#![allow(unused)]
#![warn(unused_imports)]

use crate::prelude::*;

use crate::compilation::SharedDatabase;
use crate::database::{AbsolutePath, GetResult, Table, VersionedCell};
use crate::ir::ast;
use crate::parser::parse;

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
    ) -> GetResult<VersionedCell<ast::Module>> {
        let file = db.get_file(key)?;
        let index = &self.index;

        validate! { index[key] = compute(file) }
    }
}

fn compute(file: &VersionedCell<FileMap>) -> GetResult<ast::Module> {
    let start = file.value().span().start().to_usize();
    GetResult::value(parse(file.value().src(), start)?)
}
