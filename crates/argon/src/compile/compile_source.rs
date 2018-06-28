use crate::compile_module;
use crate::parser::parse;
use failure::Error;
use parity_wasm::elements::Serialize;
use std::io::prelude::*;
use std::io::Cursor;
use std::path::Path;

pub fn compile_source(
    filename: &Path,
    mut source: impl Read,
) -> Result<impl Read + std::fmt::Debug + 'static, Error> {
    let mut content = String::new();
    {
        source.read_to_string(&mut content)?;
    }
    let ast = parse(&content).unwrap();
    let module = compile_module(&ast)?;

    let vec = Vec::new();
    let mut cursor = Cursor::new(vec);
    module.serialize(&mut cursor)?;

    cursor.set_position(0);

    Ok(cursor)
}
