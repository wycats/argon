#![allow(unused)]
#![warn(unused_imports)]

use crate::prelude::*;

use crate::annotated;
use crate::compilation::SharedDatabase;
use crate::compile::function::compile_function;
use crate::database::{AbsolutePath, GetReifyResult, GetResult, Table, ValueResult, VersionedCell};
use parity_wasm::{builder, elements};

pub struct WasmTable {
    index: Table<AbsolutePath, elements::Module>,
}

impl WasmTable {
    crate fn new() -> WasmTable {
        WasmTable {
            index: Table::new(),
        }
    }

    crate fn get(
        &self,
        mut db: SharedDatabase,
        key: &AbsolutePath,
    ) -> GetResult<VersionedCell<elements::Module>, Error> {
        let typed = db.tables().typed().get(db.clone(), key)?;

        match typed {
            ValueResult::NewValue(typed) => {
                let file = db.get_file(key)?;
                let wasm = compile(typed, file)?;
                let wasm = VersionedCell::new(wasm);
                let wasm = self.index.insert_shared(key.clone(), wasm);
                GetResult::value(wasm)
            }

            ValueResult::ValidCache => GetResult::ValueResult(ValueResult::ValidCache),
        }
    }

    crate fn get_reify(
        &self,
        db: SharedDatabase,
        key: &AbsolutePath,
    ) -> GetReifyResult<VersionedCell<elements::Module>, Error> {
        match self.get(db, key)? {
            ValueResult::NewValue(file) => GetReifyResult::ValueResult(file),
            ValueResult::ValidCache => {
                let value = self.index.get(key)?;
                GetReifyResult::ValueResult(value.weak())
            }
        }
    }
}

// impl MapTableTrait<'parent, 'entry> for WasmTable {
//     type InnerTable = DatabaseWithoutWasm<'entry>;
//     type Key = AbsolutePath;
//     type Value = elements::Module;

//     fn needs_update(&self, key: &AbsolutePath, db: &mut Self::InnerTable) -> bool {
//         let (typed, mut db) = db.without_typed();
//         let result = typed.needs_update(key, &mut db);
//         result
//     }

//     fn get(
//         &mut self,
//         db: &mut Self::InnerTable,
//         key: &Self::Key,
//         transaction: TransactionId,
//     ) -> Result<Option<Cow<Self::Value>>, Error> {
//         let needs_update = self.needs_update(key, db);

//         if needs_update {
//             self.refresh_cache(key, db, transaction)?;
//         }

//         let entry = self.index.get(key).unwrap();

//         Ok(Some(Cow::Borrowed(entry.value())))
//     }
// }

// impl CachedMapTrait<'parent, 'entry> for WasmTable {
//     fn insert(&mut self, key: Self::Key, value: Entry<Self::Value>) -> Result<(), Error> {
//         self.index.insert(key, value);
//         Ok(())
//     }

//     fn get_entry(&'entry mut self, key: &Self::Key) -> Option<&'entry Entry<Self::Value>> {
//         self.index.get(key)
//     }

//     fn compute_cache(
//         key: &AbsolutePath,
//         mut db: &mut DatabaseWithoutWasm<'inner>,
//         transaction: TransactionId,
//     ) -> Result<Option<Self::Value>, Error>
//     where
//         Error: 'static,
//     {
//         let module: annotated::Module = {
//             let (typed, mut rest) = db.without_typed();

//             try_option!(typed.get(&mut rest, key, transaction)?).into_owned()
//         };

//         let (typed, mut rest) = db.without_typed();

//         let mut code = rest.code();
//         let file = try_result!(code.get(&mut (), key, transaction));

//         Ok(Some(compile(&module, file.into_owned())?))
//     }
// }

struct CodeLocation {
    /// Location (index in 'functions' section) of the signature
    signature: u32,
    /// Location (index in the 'code' section) of the body
    _body: u32,
}

fn compile(
    module: VersionedCell<annotated::Module>,
    file: VersionedCell<FileMap>,
) -> Result<elements::Module, Error> {
    let mut builder = builder::module();
    let module = module.value();
    let file = file.value();

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
