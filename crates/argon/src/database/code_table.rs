use code_database::{AbsolutePath, FileTable, FileTrait, TransactionId};
use codespan::{CodeMap, FileMap, FileName};
use failure::Error;
use std::collections::btree_map::Entry as BTreeEntry;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::sync::Arc;

#[derive(Debug)]
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
        table: &FileTable<impl FileTrait>,
        key: &AbsolutePath,
    ) -> Option<usize> {
        table.get_entry_revision(key)
    }

    crate fn is_valid(
        &self,
        table: &FileTable<impl FileTrait>,
        key: &AbsolutePath,
        _revision: usize,
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
        trace!(target: "argon::code_table", "getting {:?}", key);
        self.refresh_cache(key, table, transaction)?;

        match self.index.get(key) {
            None => Ok(None),
            Some(entry) => Ok(Some(entry.file.clone())),
        }
    }

    fn refresh_cache(
        &mut self,
        key: &AbsolutePath,
        file_table: &mut FileTable<impl FileTrait>,
        transaction: TransactionId,
    ) -> Result<(), Error> {
        match self.index.entry(key.clone()) {
            BTreeEntry::Vacant(vacant) => {
                fill_cache(
                    BTreeEntry::Vacant(vacant),
                    &mut self.codemap,
                    file_table,
                    transaction,
                )?;
            }
            BTreeEntry::Occupied(occupied) => {
                let last_revision = {
                    let entry = occupied.get();
                    entry.last_revision
                };

                if !file_table.is_valid(key, last_revision) {
                    fill_cache(
                        BTreeEntry::Occupied(occupied),
                        &mut self.codemap,
                        file_table,
                        transaction,
                    )?;
                }
            }
        }

        Ok(())
    }
}

fn fill_cache(
    entry: BTreeEntry<AbsolutePath, Entry>,
    codemap: &mut codespan::CodeMap,
    file_table: &mut FileTable<impl FileTrait>,
    transaction: TransactionId,
) -> Result<(), Error> {
    let file = match file_table.get_entry_value_mut(entry.key(), transaction) {
        None => return Ok(()),
        Some(file) => file,
    };

    let mut body = String::new();
    file.read()?.read_to_string(&mut body)?;

    let filemap = codemap.add_filemap(FileName::Real(entry.key().as_path_buf().clone()), body);
    let new_entry = Entry {
        file: filemap,
        last_revision: 0,
    };

    match entry {
        BTreeEntry::Occupied(mut occupied) => {
            occupied.insert(new_entry);
        }
        BTreeEntry::Vacant(vacant) => {
            vacant.insert(new_entry);
        }
    };

    Ok(())
}
