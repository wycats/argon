#![allow(unused)]
#![warn(unused_imports)]

use code_database::{AbsolutePath, TransactionId};
use codespan::FileMap;
use crate::annotated;
use crate::compilation::DatabaseWithoutWasm;
use crate::compile::function::compile_function;
use crate::database::MapTableTrait;
use failure::Error;
use parity_wasm::{builder, elements};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::Arc;

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
}

impl MapTableTrait<'parent, 'entry> for WasmTable {
    type InnerTable = DatabaseWithoutWasm<'entry>;
    type Key = AbsolutePath;
    type Value = elements::Module;

    fn needs_update(&self, key: &AbsolutePath, db: &mut Self::InnerTable) -> bool {
        let (typed, mut db) = db.without_typed();
        let result = typed.needs_update(key, &mut db);
        result
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
    mut db: &mut DatabaseWithoutWasm<'inner>,
    transaction: TransactionId,
) -> Result<Option<Entry>, Error>
where
    Error: 'static,
{
    let module: annotated::Module = {
        let (typed, mut rest) = db.without_typed();

        let module: Cow<annotated::Module> = match typed.get(&mut rest, key, transaction)? {
            None => return Ok(None),
            Some(module) => module,
        };

        module.into_owned()
    };

    let (typed, mut rest) = db.without_typed();

    let mut code = rest.code();

    let file = code
        .get(&mut (), key, transaction)?
        .expect(&format!("Expected FileMap for {:?}", key));

    let module = compile(&module, file.into_owned())?;

    let new_entry = Entry {
        module: module.clone(),
        last_revision: 0,
    };

    // match entry {
    //     BTreeEntry::Occupied(mut occupied) => {
    //         occupied.insert(new_entry);
    //     }
    //     BTreeEntry::Vacant(vacant) => {
    //         vacant.insert(new_entry);
    //     }
    // };

    Ok(Some(new_entry))
}

struct CodeLocation {
    /// Location (index in 'functions' section) of the signature
    signature: u32,
    /// Location (index in the 'code' section) of the body
    _body: u32,
}

fn compile(module: &annotated::Module, file: Arc<FileMap>) -> Result<elements::Module, Error> {
    let mut builder = builder::module();

    for func in &module.funcs {
        let function = builder::function();
        let function = compile_function(function, func);
        let location: CodeLocation =
            unsafe { std::mem::transmute(builder.push_function(function)) };

        let name_span = func.name.span.to_codespan_span();
        let name = file.src_slice(name_span)?;

        if func.modifiers.export {
            builder = builder
                .export()
                .field(name)
                .internal()
                .func(location.signature)
                .build();
        }
    }

    Ok(builder.build())
}
