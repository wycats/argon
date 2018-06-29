use crate::database::{TableId, TransactionId};
use crate::entry::Entry;
use std::sync::atomic::{AtomicUsize, Ordering};

#[cfg(test)]
mod tests;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
pub struct EntryId(TableId, usize);

pub enum EntrySlot<E: Entry> {
    Pointer(EntryId),
    Entry {
        entry: E,
        last_consumed: AtomicUsize,
    },
    Placeholder,
}

impl<E: Entry> EntrySlot<E> {
    fn as_pointer(&self) -> usize {
        match self {
            EntrySlot::Pointer(EntryId(_, id)) => *id,
            EntrySlot::Entry { .. } => panic!("Unexpected entry"),
            EntrySlot::Placeholder => panic!("Unexpected placeholder"),
        }
    }

    fn as_entry(&self, transaction: TransactionId) -> &E {
        match self {
            EntrySlot::Pointer(..) => panic!("Unexpected pointer"),
            EntrySlot::Placeholder => panic!("Unexpected placeholder"),

            EntrySlot::Entry {
                entry,
                last_consumed,
            } => {
                last_consumed.store(transaction.0, Ordering::SeqCst);
                entry
            }
        }
    }
}

#[derive(new)]
pub struct Table<E: Entry> {
    #[new(default)]
    rows: Vec<EntrySlot<E>>,
    table_id: TableId,
    #[new(default)]
    next_entry: usize,
}

impl<E: Entry> Table<E> {
    pub fn add(&mut self, entry: E, transaction_id: TransactionId) -> EntryId {
        let next_entry = self.next_entry;

        if next_entry == self.rows.len() {
            self.next_entry += 1;
            self.rows.push(EntrySlot::Placeholder);
        } else {
            let prev = self.rows[next_entry].as_pointer();
            self.next_entry = prev;
        }

        self.rows[next_entry] = EntrySlot::Entry {
            entry,
            last_consumed: AtomicUsize::new(transaction_id.0),
        };

        self.entry_id(next_entry)
    }

    pub fn borrow(&self, id: EntryId, transaction: TransactionId) -> &E {
        debug_assert!(
            id.0 == self.table_id,
            "Wrong TableId (passed {:?}, this table was {:?})",
            id.0,
            self.table_id
        );

        &self.rows[id.1].as_entry(transaction)
    }

    pub fn deref(&self, id: EntryId, transaction: TransactionId) -> E
    where
        E: Copy,
    {
        *self.borrow(id, transaction)
    }

    pub fn drop(&mut self, id: EntryId) {
        let next_entry = self.entry_id(self.next_entry);
        self.rows[id.1] = EntrySlot::Pointer(next_entry);
        self.next_entry = id.1;
    }

    fn entry_id(&self, id: usize) -> EntryId {
        EntryId(self.table_id, id)
    }
}
