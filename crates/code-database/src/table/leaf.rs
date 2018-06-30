use crate::database::{TableId, TransactionId};
use crate::entry::Entry;
use crate::table::Table;
use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::mem::replace;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
pub struct EntryId(TableId, usize);

#[derive(Debug)]
pub struct InsertedEntry<E: Entry> {
    entry: E,
    last_consumed: AtomicUsize,
}

#[derive(Debug)]
pub enum EntrySlot<E: Entry> {
    Pointer(EntryId),
    Entry(InsertedEntry<E>),
    Placeholder,
}

impl<E: Entry> EntrySlot<E> {
    fn as_pointer(&self) -> usize {
        match self {
            EntrySlot::Pointer(EntryId(_, id)) => *id,
            EntrySlot::Entry(..) => panic!("Unexpected entry"),
            EntrySlot::Placeholder => panic!("Unexpected placeholder"),
        }
    }

    fn as_entry(&self, transaction: TransactionId) -> &E {
        match self {
            EntrySlot::Pointer(..) => panic!("Unexpected pointer"),
            EntrySlot::Placeholder => panic!("Unexpected placeholder"),

            EntrySlot::Entry(InsertedEntry {
                entry,
                last_consumed,
            }) => {
                last_consumed.store(transaction.0, Ordering::SeqCst);
                entry
            }
        }
    }

    fn as_entry_mut(&mut self, transaction: TransactionId) -> &mut E {
        match self {
            EntrySlot::Pointer(..) => panic!("Unexpected pointer"),
            EntrySlot::Placeholder => panic!("Unexpected placeholder"),

            EntrySlot::Entry(InsertedEntry {
                entry,
                last_consumed,
            }) => {
                last_consumed.store(transaction.0, Ordering::SeqCst);
                entry
            }
        }
    }

    fn peek(&self) -> &E {
        match self {
            EntrySlot::Pointer(..) => panic!("Unexpected pointer"),
            EntrySlot::Placeholder => panic!("Unexpected placeholder"),

            EntrySlot::Entry(InsertedEntry { entry: e, .. }) => e,
        }
    }
}

#[derive(new, Debug)]
pub struct LeafTable<E: Entry> {
    table_id: TableId,
    #[new(default)]
    rows: Vec<EntrySlot<E>>,
    #[new(default)]
    next_entry: usize,
    #[new(default)]
    index: BTreeMap<E::Key, EntryId>,
}

impl<E: Entry> LeafTable<E> {
    pub fn get_entry_value(
        &self,
        key: impl Borrow<E::Key>,
        id: TransactionId,
    ) -> Option<&E::Value> {
        let entry = self.get_entry(key, id)?;
        Some(entry.value(id))
    }

    pub fn get_entry_value_mut(
        &mut self,
        key: impl Borrow<E::Key>,
        id: TransactionId,
    ) -> Option<&mut E::Value> {
        let entry = self.get_entry_mut(key, id)?;
        Some(entry.value_mut(id))
    }

    pub fn deref_entry_value(&self, key: impl Borrow<E::Key>, id: TransactionId) -> Option<E::Value>
    where
        E::Value: Copy,
    {
        self.get_entry_value(key.borrow(), id).map(|v| *v)
    }

    pub fn get_entry_tag(&self, key: impl Borrow<E::Key>) -> Option<E::Tag> {
        let entry = self.peek_entry_by_key(key.borrow())?;
        Some(entry.tag())
    }

    pub fn get_entry_revision(&self, key: impl Borrow<E::Key>) -> Option<usize> {
        let entry = self.peek_entry_by_key(key.borrow())?;
        Some(crate::tag::Tag::revision(&entry.tag()))
    }

    // TODO: This should take an E::Hash, not the last value, which would be unnecessarily
    // expensive to store
    pub fn can_reuse_entry(&self, key: &E::Key, snapshot: usize, last_value: &E::Value) -> bool {
        match self.peek_entry_by_key(key) {
            Some(entry) => entry.peek(snapshot, last_value),
            None => false,
        }
    }

    pub fn is_valid(&self, key: &E::Key, snapshot: usize) -> bool {
        match self.get_entry_tag(key) {
            Some(tag) => crate::tag::Tag::is_valid(&tag, snapshot),
            None => false,
        }
    }

    pub fn add_entry(&mut self, entry: E, transaction_id: TransactionId) -> EntryId {
        let next_entry = self.next_entry;

        if next_entry == self.rows.len() {
            self.next_entry += 1;
            self.rows.push(EntrySlot::Placeholder);
        } else {
            let prev = self.rows[next_entry].as_pointer();
            self.next_entry = prev;
        }

        let key = entry.key().clone();
        self.rows[next_entry] = EntrySlot::Entry(InsertedEntry {
            entry,
            last_consumed: AtomicUsize::new(transaction_id.0),
        });

        let id = self.entry_id(next_entry);
        self.index.insert(key, id);

        self.entry_id(next_entry)
    }

    pub fn peek_entry(&self, id: EntryId) -> &E {
        debug_assert!(
            id.0 == self.table_id,
            "Wrong TableId (passed {:?}, this table was {:?})",
            id.0,
            self.table_id
        );

        &self.rows[id.1].peek()
    }

    pub fn deref_entry(&self, id: EntryId, transaction: TransactionId) -> E
    where
        E: Copy,
    {
        *self.borrow_entry(id, transaction)
    }

    pub fn borrow_entry(&self, id: EntryId, transaction: TransactionId) -> &E {
        debug_assert!(
            id.0 == self.table_id,
            "Wrong TableId (passed {:?}, this table was {:?})",
            id.0,
            self.table_id
        );

        &self.rows[id.1].as_entry(transaction)
    }

    // dropping should only be done by a sweep at the end of a transaction, which
    // ensures that everyone who wants to consume this entry has a chance
    pub fn drop_entry(&mut self, id: EntryId) {
        let next_entry = self.entry_id(self.next_entry);
        let old_entry = replace(&mut self.rows[id.1], EntrySlot::Pointer(next_entry));
        let old_entry = old_entry.peek();
        self.index.remove(old_entry.key());

        self.next_entry = id.1;
    }

    pub fn get_entry(&self, key: impl Borrow<E::Key>, transaction: TransactionId) -> Option<&E> {
        let id = self.index.get(key.borrow())?;

        Some(&self.rows[id.1].as_entry(transaction))
    }

    pub fn get_entry_mut(
        &mut self,
        key: impl Borrow<E::Key>,
        transaction: TransactionId,
    ) -> Option<&mut E> {
        let id = self.index.get(key.borrow())?;

        Some(self.rows[id.1].as_entry_mut(transaction))
    }

    pub fn peek_entry_by_key(&self, key: &E::Key) -> Option<&E> {
        let id = self.index.get(key.borrow())?;

        Some(&self.rows[id.1].peek())
    }

    fn entry_id(&self, id: usize) -> EntryId {
        EntryId(self.table_id, id)
    }
}

impl<E: Entry> Table for LeafTable<E> {
    type Key = E::Key;
    type Value = E::Value;
    type Tag = E::Tag;

    fn get_table_value_by_key(&self, key: &Self::Key, id: TransactionId) -> Option<&Self::Value> {
        self.get_entry_value(key, id)
    }

    fn get_table_tag_by_key(&self, key: &Self::Key) -> Option<Self::Tag> {
        self.get_entry_tag(key)
    }
}
