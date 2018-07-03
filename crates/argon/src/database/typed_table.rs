#![allow(unused)]
#![warn(unused_imports)]

use code_database::{AbsolutePath, TransactionId};
use crate::annotated;
use crate::compilation::DatabaseWithoutTyped;
use crate::database::MapTableTrait;
use crate::infer::UnifyTable;
use crate::ir::{ast, resolved};
use failure::Error;
use std::borrow::{Borrow, Cow};
use std::collections::BTreeMap;

crate struct Entry {
    module: annotated::Module,
    last_revision: usize,
}

impl Entry {
    fn module(&self) -> &annotated::Module {
        &self.module
    }
}

crate struct TypedTable {
    index: BTreeMap<AbsolutePath, Entry>,
}

impl TypedTable {
    crate fn new() -> TypedTable {
        TypedTable {
            index: BTreeMap::new(),
        }
    }
}

impl MapTableTrait<'parent, 'entry> for TypedTable {
    type InnerTable = DatabaseWithoutTyped<'entry>;
    type Key = AbsolutePath;
    type Value = annotated::Module;

    fn needs_update(&self, key: &AbsolutePath, mut db: &mut Self::InnerTable) -> bool {
        let (mut db, ast) = db.parts();
        ast.needs_update(key, &mut db)
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

        let entry = self.index.get(key).unwrap();

        Ok(Some(Cow::Borrowed(&entry.module)))
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

fn compute_cache(
    key: &AbsolutePath,
    mut db: &mut DatabaseWithoutTyped,
    transaction: TransactionId,
) -> Result<Option<Entry>, Error> {
    let (mut db, ast) = db.parts();

    let ast: Cow<ast::Module> = match ast.get(&mut db, key, transaction)? {
        None => return Ok(None),
        Some(file) => file,
    };

    let module = compile(ast.borrow())?;

    let new_entry = Entry {
        module,
        last_revision: 0,
    };

    Ok(Some(new_entry))
}

fn compile(ast: &ast::Module) -> Result<annotated::Module, Error> {
    let mut table = UnifyTable::new();

    let module = resolved::resolve_module_names(&ast)?;
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
