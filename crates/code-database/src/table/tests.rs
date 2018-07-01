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
        assert_eq!(
            self.table
                .deref_entry_value(key, self.transaction)
                .expect(&format!("missing key={:?}", key)),
            value,
            "deref {:?} = {:?}",
            key,
            value
        );

        assert_eq!(
            self.table
                .get_entry_value(key, self.transaction)
                .expect(&format!("missing key={:?}", key)),
            &value,
            "get value {:?} = {:?}",
            key,
            value
        );

        assert_eq!(
            self.table
                .get_entry(key, self.transaction)
                .expect(&format!("missing key={:?}", key)),
            &value,
            "get entry {:?} = {:?}",
            key,
            value
        );
        assert_eq!(
            self.table.deref_entry(entry, self.transaction),
            value,
            "deref entry {:?} = {:?}",
            entry,
            value
        );
        assert_eq!(
            self.table.borrow_entry(entry, self.transaction),
            &value,
            "borrow entry {:?} = {:?}",
            entry,
            value
        );

        Ok(())
    }
}

#[test]
fn test_storage() -> Result<(), NoneError> {
    let mut database = Database::new();
    let mut table = database.new_table(|i| LeafTable::new(i));
    let transaction = database.begin();

    let e1 = table.add_entry("1", transaction);
    let e2 = table.add_entry("2", transaction);
    let e3 = table.add_entry("3", transaction);
    let e4 = table.add_entry("4", transaction);

    {
        let expect = expectation(&table, transaction);
        expect.entry(e1).key("1").value("1")?;
        expect.entry(e2).key("2").value("2")?;
        expect.entry(e3).key("3").value("3")?;
        expect.entry(e4).key("4").value("4")?;
    }

    table.drop_entry(e2);

    assert_eq!(table.get_entry("2", transaction), None);

    let e2 = table.add_entry("2b", transaction);

    {
        let expect = expectation(&table, transaction);
        expect.entry(e1).key("1").value("1")?;
        expect.entry(e2).key("2b").value("2b")?;
        expect.entry(e3).key("3").value("3")?;
        expect.entry(e4).key("4").value("4")?;
    }

    table.drop_entry(e1);
    table.drop_entry(e2);
    table.drop_entry(e4);
    table.drop_entry(e3);

    let e1 = table.add_entry("1c", transaction);
    let e2 = table.add_entry("2c", transaction);
    let e3 = table.add_entry("3c", transaction);
    let e4 = table.add_entry("4c", transaction);

    {
        let expect = expectation(&table, transaction);
        expect.entry(e1).key("1c").value("1c")?;
        expect.entry(e2).key("2c").value("2c")?;
        expect.entry(e3).key("3c").value("3c")?;
        expect.entry(e4).key("4c").value("4c")?;
    }

    Ok(())
}
