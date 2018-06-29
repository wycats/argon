use crate::database::{Database, TableId};
use crate::entry::Entry;
use crate::tag::Tag;

use super::*;

pub struct StaticTag;

impl Tag for StaticTag {
    fn revision(&self) -> u64 {
        0
    }

    fn is_valid(&self, snapshot: u64) -> bool {
        true
    }
}

impl Entry for &'static str {
    type Tag = StaticTag;
    type Value = &'static str;

    fn tag(&self) -> StaticTag {
        StaticTag
    }

    fn value(&self) -> &&'static str {
        self
    }

    fn peek(&self, _snapshot: u64, _last_value: &Self::Value) -> bool {
        true
    }
}

#[test]
fn test_storage() {
    let mut database = Database::new();
    let mut table = Table::new(database.new_table());
    let transaction = database.begin();

    let e1 = table.add("1", transaction);
    let e2 = table.add("2", transaction);
    let e3 = table.add("3", transaction);
    let e4 = table.add("4", transaction);

    assert_eq!(table.deref(e1, transaction), "1");
    assert_eq!(table.deref(e2, transaction), "2");
    assert_eq!(table.deref(e3, transaction), "3");
    assert_eq!(table.deref(e4, transaction), "4");

    table.drop(e2);

    let e2 = table.add("2b", transaction);

    assert_eq!(table.deref(e2, transaction), "2b");

    table.drop(e1);
    table.drop(e2);
    table.drop(e4);
    table.drop(e3);

    let e1 = table.add("1c", transaction);
    let e2 = table.add("2c", transaction);
    let e3 = table.add("3c", transaction);
    let e4 = table.add("4c", transaction);

    assert_eq!(table.deref(e1, transaction), "1c");
    assert_eq!(table.deref(e2, transaction), "2c");
    assert_eq!(table.deref(e3, transaction), "3c");
    assert_eq!(table.deref(e4, transaction), "4c");
}
