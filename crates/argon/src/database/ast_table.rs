#![allow(unused)]
#![warn(unused_imports)]

use crate::compilation::SharedDatabase;
use crate::database::VersionedCell;
use crate::database::{AbsolutePath, GetResult, Table, ValueResult};
use crate::ir::ast;
use crate::parser::parse;
use failure::Error;
use std::sync::Weak;

pub struct AstTable {
    index: Table<AbsolutePath, ast::Module>,
}

impl AstTable {
    crate fn new() -> AstTable {
        AstTable {
            index: Table::new(),
        }
    }
}

impl AstTable {
    crate fn get(
        &self,
        db: SharedDatabase,
        key: &AbsolutePath,
    ) -> GetResult<VersionedCell<ast::Module>, Error> {
        let file = db.get_file(key)?;

        let parsed = parse(file.value().src())?;
        let parsed = VersionedCell::new(parsed);
        let parsed = self.index.insert_shared(key.clone(), parsed);
        GetResult::value(parsed)
    }

    crate fn get_reify(
        &self,
        db: SharedDatabase,
        key: &AbsolutePath,
    ) -> GetResult<VersionedCell<ast::Module>, Error> {
        match self.get(db, key)? {
            ValueResult::NewValue(file) => GetResult::value(file),
            ValueResult::ValidCache => {
                let value = self.index.get(key)?;
                GetResult::value(value)
            }
        }
    }
}
