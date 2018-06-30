use code_database::{AbsolutePath, FileTable, FileTrait, TransactionId};
use codespan::{CodeMap, FileMap, FileName};
use failure::Error;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::sync::Arc;

crate struct Entry {
    file: Arc<FileMap>,
    last_revision: usize,
}

crate struct CodeTable {
    codemap: codespan::CodeMap,
    index: BTreeMap<AbsolutePath, Entry>,
}

impl CodeTable {
    crate fn new() -> CodeTable {
        CodeTable {
            codemap: CodeMap::new(),
            index: BTreeMap::new(),
        }
    }

    crate fn get_revision(
        &self,
        table: &mut FileTable<impl FileTrait>,
        key: &AbsolutePath,
    ) -> Option<usize> {
        table.get_entry_revision(key)
    }

    crate fn is_valid(
        &self,
        table: &mut FileTable<impl FileTrait>,
        key: &AbsolutePath,
        revision: usize,
    ) -> bool {
        let entry = match self.index.get(key) {
            None => return false,
            Some(entry) => entry,
        };

        table.is_valid(key, entry.last_revision)
    }

    crate fn get(
        &mut self,
        table: &mut FileTable<impl FileTrait>,
        key: &AbsolutePath,
        transaction: TransactionId,
    ) -> Result<Option<Arc<FileMap>>, Error> {
        let entry = match self.index.get(key) {
            None => return Ok(None),
            Some(entry) => entry,
        };

        if table.is_valid(key, entry.last_revision) {
            return Ok(Some(entry.file.clone()));
        }

        let revision = table.get_entry_revision(key);
        let file = match table.get_entry_value_mut(key, transaction) {
            None => return Ok(None),
            Some(file) => file,
        };

        let mut body = String::new();
        file.read()?.read_to_string(&mut body)?;

        let filemap = self
            .codemap
            .add_filemap(FileName::Real(key.as_path_buf().clone()), body);

        Ok(Some(filemap))
    }
}
