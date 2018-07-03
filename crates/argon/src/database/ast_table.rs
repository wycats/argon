#![allow(unused)]
#![warn(unused_imports)]

use super::codemap_table::CodemapTable;
use code_database::{AbsolutePath, TransactionId};
use crate::database::MapTableTrait;
use crate::ir::ast::Module;
use crate::parser::parse;
use failure::Error;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Debug)]
crate struct Entry {
    module: Module,
    last_revision: usize,
}

crate struct AstTable {
    index: BTreeMap<AbsolutePath, Entry>,
}

impl AstTable {
    crate fn new() -> AstTable {
        AstTable {
            index: BTreeMap::new(),
        }
    }
}

fn compute_cache(
    key: &AbsolutePath,
    db: &mut CodemapTable,
    transaction: TransactionId,
) -> Result<Option<Entry>, Error> {
    let CodemapTable { files, code } = db;

    let file = match code.get(files.files_mut(), key, transaction)? {
        None => return Ok(None),
        Some(file) => file,
    };

    let src = file.src().to_string();
    let parsed = parse(&src)?;

    let new_entry = Entry {
        module: parsed,
        last_revision: 0,
    };

    Ok(Some(new_entry))
}

impl MapTableTrait<'parent, 'inner> for AstTable {
    type InnerTable = CodemapTable<'inner>;
    type Key = AbsolutePath;
    type Value = Module;

    fn needs_update(&self, key: &Self::Key, db: &mut Self::InnerTable) -> bool {
        db.needs_update(key, &mut ())
    }

    fn get(
        &mut self,
        db: &mut Self::InnerTable,
        key: &Self::Key,
        transaction: TransactionId,
    ) -> Result<Option<Cow<Self::Value>>, Error> {
        let needs_update = self.needs_update(key, db);

        if needs_update {
            self.refresh_cache(key, db, transaction)?;
        }

        match self.index.get(key) {
            None => Ok(None),
            Some(entry) => Ok(Some(Cow::Borrowed(&entry.module))),
        }
    }

    fn refresh_cache(
        &mut self,
        key: &Self::Key,
        db: &mut Self::InnerTable,
        transaction: TransactionId,
    ) -> Result<Option<()>, Error> {
        let cache_entry = compute_cache(key, db, transaction)?;

        match cache_entry {
            Some(cache_entry) => self.index.insert(key.clone(), cache_entry),
            None => return Ok(None),
        };

        Ok(Some(()))
    }
}
