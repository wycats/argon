#![allow(unused)]
#![warn(unused_imports)]

use crate::prelude::*;

use crate::annotated;
use crate::compilation::SharedDatabase;
use crate::compile::function::compile_function;
use crate::database::{AbsolutePath, GetResult, Table, VersionedCell};
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
        let file = db.get_file(key)?;
        let index = &self.index;

        validate! { index[key] = compute(typed, file) }
    }
}

struct CodeLocation {
    /// Location (index in 'functions' section) of the signature
    signature: u32,
    /// Location (index in the 'code' section) of the body
    _body: u32,
}

fn compute(
    module: &VersionedCell<annotated::Module>,
    file: &VersionedCell<FileMap>,
) -> GetResult<elements::Module> {
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

    GetResult::value(builder.build())
}
