#![allow(unused)]
#![warn(unused_imports)]

use super::ast_table::AstTable;
use super::code_table::CodeTable;
use super::typed_table::TypedTable;
use code_database::{AbsolutePath, FileTable, FileTrait, TransactionId};
use crate::annotated;
use crate::compile::function::compile_function;
use failure::Error;
use parity_wasm::{builder, elements};
use std::collections::btree_map::Entry as BTreeEntry;
use std::collections::BTreeMap;

crate struct Entry {
    module: elements::Module,
    last_revision: usize,
}

crate struct WasmTable {
    index: BTreeMap<AbsolutePath, Entry>,
}

impl WasmTable {
    crate fn new() -> WasmTable {
        WasmTable {
            index: BTreeMap::new(),
        }
    }

    crate fn get(
        &'a mut self,
        file_table: &mut FileTable<impl FileTrait>,
        code_table: &mut CodeTable,
        ast_table: &mut AstTable,
        typed_table: &mut TypedTable,
        key: &AbsolutePath,
        transaction: TransactionId,
    ) -> Result<Option<&'a elements::Module>, Error> {
        self.refresh_cache(
            key,
            file_table,
            code_table,
            ast_table,
            typed_table,
            transaction,
        )?;

        let entry = self.index.get(key).unwrap();

        Ok(Some(&entry.module))
    }

    fn refresh_cache(
        &mut self,
        key: &AbsolutePath,
        file_table: &mut FileTable<impl FileTrait>,
        code_table: &mut CodeTable,
        ast_table: &mut AstTable,
        typed_table: &mut TypedTable,
        transaction: TransactionId,
    ) -> Result<(), Error> {
        match self.index.entry(key.clone()) {
            BTreeEntry::Vacant(vacant) => {
                fill_cache(
                    BTreeEntry::Vacant(vacant),
                    file_table,
                    code_table,
                    ast_table,
                    typed_table,
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
                        typed_table,
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
    typed_table: &mut TypedTable,
    transaction: TransactionId,
) -> Result<(), Error> {
    let module: &annotated::Module =
        match typed_table.get(file_table, code_table, ast_table, entry.key(), transaction)? {
            None => return Ok(()),
            Some(module) => module,
        };

    let module = compile(module)?;

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

struct CodeLocation {
    /// Location (index in 'functions' section) of the signature
    signature: u32,
    /// Location (index in the 'code' section) of the body
    _body: u32,
}

fn compile(module: &annotated::Module) -> Result<elements::Module, Error> {
    let mut builder = builder::module();

    for func in &module.funcs {
        let function = builder::function();
        let function = compile_function(function, func);
        let location: CodeLocation =
            unsafe { std::mem::transmute(builder.push_function(function)) };

        if func.modifiers.export {
            builder = builder
                .export()
                .field(&func.name.node)
                .internal()
                .func(location.signature)
                .build();
        }
    }

    Ok(builder.build())
}
