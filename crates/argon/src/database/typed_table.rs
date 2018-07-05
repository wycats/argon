#![allow(unused)]
#![warn(unused_imports)]

use crate::compilation::SharedDatabase;
use crate::database::VersionedCell;
use crate::database::{AbsolutePath, GetResult, Table, ValueResult};
use crate::infer::UnifyTable;
use crate::ir::{annotated, ast, resolved};
use crate::prelude::*;
use failure::Error;

pub struct TypedTable {
    index: Table<AbsolutePath, annotated::Module>,
}

impl TypedTable {
    crate fn new() -> TypedTable {
        TypedTable {
            index: Table::new(),
        }
    }

    crate fn get(
        &self,
        db: SharedDatabase,
        key: &AbsolutePath,
    ) -> GetResult<VersionedCell<annotated::Module>, Error> {
        let ast = db.tables().ast().get(db.clone(), key)?;

        match ast {
            ValueResult::NewValue(ast) => {
                let typed = compile(ast)?;
                let typed = VersionedCell::new(typed);
                let typed = self.index.insert_shared(key.clone(), typed);
                GetResult::value(typed)
            }

            ValueResult::ValidCache => GetResult::ValueResult(ValueResult::ValidCache),
        }
    }

    crate fn get_reify(
        &self,
        db: SharedDatabase,
        key: &AbsolutePath,
    ) -> GetResult<VersionedCell<annotated::Module>, Error> {
        match self.get(db, key)? {
            ValueResult::NewValue(file) => GetResult::value(file),
            ValueResult::ValidCache => {
                let value = self.index.get(key)?;
                GetResult::value(value.weak())
            }
        }
    }
}

fn compile(ast: VersionedCell<ast::Module>) -> Result<annotated::Module, Error> {
    let mut table = UnifyTable::new();

    let module = resolved::resolve_module_names(&ast.value())?;
    let module = annotated::Module::from(module, &mut table);
    trace!(target: "argon::compile::module", "Module: {:#?}", module);
    let constraints = module.constraints();
    trace!(target: "argon::compile::constraints", "Constraints: {:#?}", constraints);
    let substitutions = table.unify(constraints)?;

    trace!(target: "argon::compile::substitutions", "Substitutions: {:#?}", substitutions);
    let module = substitutions.apply_module(module);
    trace!(target: "argon::compile::applies", "After Substitutions: {:#?}", module);

    Ok(module)
}
