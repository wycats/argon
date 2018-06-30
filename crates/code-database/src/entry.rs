use crate::database::TransactionId;
use crate::tag::Tag;

pub trait Entry {
    type Tag: Tag;
    type Value;
    type Key: Ord + Clone;

    /// An indexing key. The key is returned by value, so it can be used as
    /// an indexing key in a BTree. As a result, it should be Copy or cheaply
    /// cloneable.
    fn key(&self) -> &Self::Key;

    /// A tag representing the validation of this entry
    fn tag(&self) -> Self::Tag;

    /// The current value, freshly computed
    fn value(&self, transaction: TransactionId) -> &Self::Value;

    /// The current value, freshly computed
    fn value_mut(&mut self, transaction: TransactionId) -> &mut Self::Value;

    /// Allows a caching consumer to ask an entry whether its cached value
    /// is up to date. If the value is up to date, the Entry is entitled to
    /// report the provided snapshot value to future consumers, and the
    /// consumer is not entitled to use the provided snapshot value for its
    /// consumers.
    fn peek(&self, _snapshot: usize, _last_value: &Self::Value) -> bool {
        false
    }
}
