use super::code_table::CodeTable;
use code_database::{AbsolutePath, FileTable, FileTrait, TransactionId};
use codespan::{CodeMap, FileMap, FileName};
use crate::ir::ast::Module;
use crate::parser::parse;
use failure::Error;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::sync::Arc;

crate struct Entry<'input> {
    module: Module<'input>,
    last_revision: usize,
}

crate struct AstTable<'input> {
    index: BTreeMap<AbsolutePath, Entry<'input>>,
}

impl AstTable<'input> {
    crate fn new() -> AstTable<'input> {
        AstTable {
            index: BTreeMap::new(),
        }
    }

    crate fn get(
        &mut self,
        code_table: &mut CodeTable,
        file_table: &mut FileTable<impl FileTrait>,
        key: &AbsolutePath,
        transaction: TransactionId,
    ) -> Result<Option<Module<'static>>, Error> {
        let entry = match self.index.get(key) {
            None => return Ok(None),
            Some(entry) => entry,
        };

        if code_table.is_valid(file_table, key, entry.last_revision) {
            return Ok(Some(entry.module.into_owned()));
        }

        let revision = code_table.get_revision(file_table, key);
        let file = match code_table.get(file_table, key, transaction)? {
            None => return Ok(None),
            Some(file) => file,
        }.clone();

        let src = file.src().to_string();
        let parsed = parse(&src)?;
        let parsed = parsed.into_owned();

        Ok(Some(parsed))
    }
}
