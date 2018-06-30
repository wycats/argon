use crate::database::TransactionId;
use crate::tag::Tag;

pub trait Table {
    type Value;
    type Key;
    type Tag: Tag;

    fn get_table_value_by_key(&self, key: &Self::Key, id: TransactionId) -> Option<&Self::Value>;
    fn get_table_tag_by_key(&self, key: &Self::Key) -> Option<Self::Tag>;
}
