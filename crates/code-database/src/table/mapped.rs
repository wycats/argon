use crate::database::TransactionId;
use crate::table::table_trait::Table;
use std::borrow::Borrow;

pub enum MapResult<'input, T: 'input> {
    Owned(T),
    Borrowed(&'input T),
}

impl<T> AsRef<T> for MapResult<'input, T> {
    fn as_ref(&self) -> &T {
        match self {
            MapResult::Owned(v) => v,
            MapResult::Borrowed(b) => b,
        }
    }
}

pub struct MappedTable<'a, Inner: Table + 'a, Output> {
    inner: &'a Inner,
    transform: for<'b> fn(&'b Inner::Value) -> MapResult<'b, Output>,
}

pub fn map<Inner, Output>(
    inner: &'a Inner,
    transform: for<'b> fn(&'b Inner::Value) -> MapResult<'b, Output>,
) -> MappedTable<'a, Inner, Output>
where
    Inner: Table + 'a,
{
    MappedTable { inner, transform }
}

impl<Inner, Output> MappedTable<'a, Inner, Output>
where
    Inner: Table + 'a,
{
    pub fn get_entry_value(
        &self,
        key: impl Borrow<Inner::Key>,
        id: TransactionId,
    ) -> Option<MapResult<Output>> {
        let inner = self.inner.get_table_value_by_key(key.borrow(), id);
        inner.map(self.transform)
    }

    pub fn deref_entry_value(
        &self,
        key: impl Borrow<Inner::Key>,
        id: TransactionId,
    ) -> Option<Output>
    where
        Output: Copy,
        Inner::Value: Copy,
    {
        let inner = self.get_entry_value(key, id);
        inner.map(|i| *i.as_ref())
    }

    pub fn get_entry_tag(&self, key: impl Borrow<Inner::Key>) -> Option<Inner::Tag> {
        self.inner.get_table_tag_by_key(key.borrow())
    }
}
