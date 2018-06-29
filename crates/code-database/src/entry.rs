use crate::tag::Tag;

pub trait Entry {
    type Tag: Tag;
    type Value;

    /// A tag representing the validation of this entry
    fn tag(&self) -> Self::Tag;

    /// The current value, freshly computed
    fn value(&self) -> &Self::Value;

    /// Allows a caching consumer to ask an entry whether its cached value
    /// is up to date. If the value is up to date, the Entry is entitled to
    /// report the provided snapshot value to future consumers, and the
    /// consumer is not entitled to use the provided snapshot value for its
    /// consumers.
    fn peek(&self, _snapshot: u64, _last_value: &Self::Value) -> bool {
        false
    }
}
