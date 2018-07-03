use code_database::TransactionId;
use failure::Error;
use std::borrow::Cow;

crate trait MapTableTrait<'parent, 'entry> {
    type InnerTable;
    type Key;
    type Value: Clone;

    fn get(
        &'entry mut self,
        db: &mut Self::InnerTable,
        key: &Self::Key,
        transaction: TransactionId,
    ) -> Result<Option<Cow<'entry, Self::Value>>, Error>;

    fn refresh_cache(
        &mut self,
        key: &Self::Key,
        db: &mut Self::InnerTable,
        transaction: TransactionId,
    ) -> Result<Option<()>, Error>;

    fn needs_update(&self, key: &Self::Key, db: &mut Self::InnerTable) -> bool;

    // fn compute_cache(
    //     key: &Self::Key,
    //     db: &mut Self::InnerTable,
    //     transaction: TransactionId,
    // ) -> Self::Value;
}
