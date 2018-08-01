#![allow(unused)]
#![warn(unused_imports)]

use crate::prelude::*;

use crate::compilation::SharedDatabase;
use crate::database::{AbsolutePath, GetResult, Table, VersionedCell};
use crate::infer::UnifyTable;
use crate::ir::{annotated, ast, resolved};
use log::*;

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
        db: SharedDatabase<'_>,
        key: &AbsolutePath,
    ) -> GetResult<VersionedCell<annotated::Module>, ArgonError> {
        let ast = db.tables().ast().get(db.clone(), key)?;
        let index = &self.index;

        validate! { index[key] = compute(ast) }
    }
}

fn compute(ast: &VersionedCell<ast::Module>) -> GetResult<annotated::Module> {
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

    GetResult::value(module)
}
