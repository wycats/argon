use crate::database::{Database, TransactionId};
use crate::entry::Entry;
use crate::table::EntryId;
use crate::tag::Tag;
use std::option::NoneError;

use super::*;

pub struct StaticTag;

impl Tag for StaticTag {
    fn revision(&self) -> usize {
        0
    }

    fn is_valid(&self, _snapshot: usize) -> bool {
        true
    }
}

impl Entry for &'static str {
    type Tag = StaticTag;
    type Key = &'static str;
    type Value = &'static str;

    fn key(&self) -> &&'static str {
        self
    }

    fn tag(&self) -> StaticTag {
        StaticTag
    }

    fn value(&self, _transaction: TransactionId) -> &&'static str {
        self
    }

    fn value_mut(&mut self, _transaction: TransactionId) -> &mut &'static str {
        self
    }

    fn peek(&self, _snapshot: usize, _last_value: &Self::Value) -> bool {
        true
    }
}

#[derive(new, Debug, Clone)]
struct Expect<'ctx> {
    table: &'ctx LeafTable<&'static str>,
    transaction: TransactionId,
    #[new(default)]
    key: Option<&'static str>,
    #[new(default)]
    value: Option<&'static str>,
    #[new(default)]
    entry: Option<EntryId>,
}

fn expectation(table: &'ctx LeafTable<&'static str>, transaction: TransactionId) -> Expect<'ctx> {
    Expect::new(table, transaction)
}

impl Expect<'ctx> {
    fn entry(&self, e: EntryId) -> Expect<'ctx> {
        let mut expect = self.clone();
        expect.entry = Some(e);
        expect
    }

    fn key(&self, s: &'static str) -> Expect<'ctx> {
        let mut expect = self.clone();
        expect.key = Some(s);
        expect
    }

    fn value(&self, s: &'static str) -> Result<(), NoneError> {
        let mut expect = self.clone();
        expect.value = Some(s);
        expect.try_expect()
    }

    fn try_expect(&self) -> Result<(), NoneError> {
        match (self.key, self.value, self.entry) {
            (Some(key), Some(value), Some(entry)) => self.expect(key, value, entry),
            _ => panic!("Wrong order, call value last: {:?}", self),
        }
    }

    fn expect(
        &self,
        key: &'static str,
        value: &'static str,
        entry: EntryId,
    ) -> Result<(), NoneError> {
        println!("{:?} {:?} {:?}", key, value, entry);

        let actual = self.table.get_entry(key, self.transaction);
        let actual = actual.unwrap();
        let actual = actual.lock().unwrap();

        assert_eq!(*actual, value, "deref {:?} = {:?}", key, value);

        let table_entry = self
            .table
            .get_entry(key, self.transaction)
            .expect(&format!("missing key={:?}", key));

        let actual = table_entry.lock().unwrap();

        assert_eq!(*actual, value, "get entry {:?} = {:?}", key, value);
        assert_eq!(
            self.table.deref_entry(entry, self.transaction),
            value,
            "deref entry {:?} = {:?}",
            entry,
            value
        );

        let table_entry = self.table.borrow_entry(entry, self.transaction);
        let actual = table_entry.lock().unwrap();

        assert_eq!(*actual, value, "borrow entry {:?} = {:?}", entry, value);

        Ok(())
    }
}
