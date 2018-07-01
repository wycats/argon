#![allow(unused)]
#![warn(unused_imports)]

use super::ast_table::AstTable;
use super::code_table::CodeTable;
use code_database::{AbsolutePath, FileTable, FileTrait, TransactionId};
use crate::annotated;
use crate::infer::UnifyTable;
use crate::ir::{ast, resolved};
use failure::Error;
use std::collections::btree_map::Entry as BTreeEntry;
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

    crate fn get(
        &'a mut self,
        file_table: &mut FileTable<impl FileTrait>,
        code_table: &mut CodeTable,
        ast_table: &mut AstTable,
        key: &AbsolutePath,
        transaction: TransactionId,
    ) -> Result<Option<&'a annotated::Module>, Error> {
        self.refresh_cache(key, file_table, code_table, ast_table, transaction)?;

        let entry = self.index.get(key).unwrap();

        Ok(Some(&entry.module))
    }

    fn refresh_cache(
        &mut self,
        key: &AbsolutePath,
        file_table: &mut FileTable<impl FileTrait>,
        code_table: &mut CodeTable,
        ast_table: &mut AstTable,
        transaction: TransactionId,
    ) -> Result<(), Error> {
        match self.index.entry(key.clone()) {
            BTreeEntry::Vacant(vacant) => {
                fill_cache(
                    BTreeEntry::Vacant(vacant),
                    file_table,
                    code_table,
                    ast_table,
                    transaction,
                )?;
            }
            BTreeEntry::Occupied(occupied) => {
                let last_revision = {
                    let entry = occupied.get();
                    entry.last_revision
                };

                if !code_table.is_valid(file_table, key, last_revision) {
                    fill_cache(
                        BTreeEntry::Occupied(occupied),
                        file_table,
                        code_table,
                        ast_table,
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
    file_table: &mut FileTable<impl FileTrait>,
    code_table: &mut CodeTable,
    ast_table: &mut AstTable,
    transaction: TransactionId,
) -> Result<(), Error> {
    let ast: &ast::Module<'static> =
        match ast_table.get(file_table, code_table, entry.key(), transaction)? {
            None => return Ok(()),
            Some(file) => file,
        };

    let module = compile(ast)?;

    let new_entry = Entry {
        module,
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

fn compile(ast: &ast::Module<'static>) -> Result<annotated::Module, Error> {
    let mut table = UnifyTable::new();

    let module = resolved::resolve_module_names(&ast)?;
    let module = annotated::Module::from(module, &mut table);
    // trace!(target: "wasm::compile::module", "Module: {:#?}", module);
    let constraints = module.constraints();
    // trace!(target: "wasm::compile::constraints", "Constraints: {:#?}", constraints);
    let substitutions = table.unify(constraints)?;

    // trace!(target: "wasm::compile::substitutions", "Substitutions: {:#?}", substitutions);
    let module = substitutions.apply_module(module);
    // trace!(target: "wasm::compile::applies", "After Substitutions: {:#?}", module);

    Ok(module)
}
