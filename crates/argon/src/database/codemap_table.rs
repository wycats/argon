use super::code_table::CodeTable;
use code_database::Database as CodeDatabase;
use code_database::{AbsolutePath, TransactionId};
use codespan::FileMap;
use crate::database::MapTableTrait;
use failure::Error;
use std::borrow::Cow;
use std::sync::Arc;

crate struct CodemapTable<'parent> {
    crate files: &'parent mut CodeDatabase,
    crate code: &'parent mut CodeTable,
}

impl CodemapTable<'parent> {
    crate fn is_valid(&self, key: &AbsolutePath) -> bool {
        let CodemapTable { files, code } = self;
        let last_revision = match code.get_revision(files.files(), key) {
            Some(revision) => revision,
            None => return false,
        };

        code.is_valid(files.files(), key, last_revision)
    }
}

impl MapTableTrait<'parent, 'entry> for CodemapTable<'parent> {
    type InnerTable = ();
    type Key = AbsolutePath;
    type Value = Arc<FileMap>;

    fn get(
        &'entry mut self,
        _db: &mut Self::InnerTable,
        key: &AbsolutePath,
        transaction: TransactionId,
    ) -> Result<Option<Cow<Arc<FileMap>>>, Error> {
        let CodemapTable { files, code } = self;

        let file = match code.get(files.files_mut(), key, transaction)? {
            None => return Ok(None),
            Some(file) => file,
        };

        Ok(Some(Cow::Owned(file)))
    }

    fn needs_update(&self, key: &Self::Key, _db: &mut Self::InnerTable) -> bool {
        !self.is_valid(key)
    }

    fn refresh_cache(
        &mut self,
        _key: &Self::Key,
        _db: &mut Self::InnerTable,
        _transaction: TransactionId,
    ) -> Result<Option<()>, Error> {
        Ok(Some(()))
    }
}
