crate mod body;
crate mod expression;
crate mod function;
crate mod math;

pub use self::math::*;
use crate::{AbsolutePath, ArgonError, Compilation, Database, GetResult, SkipResult};
use std::path::PathBuf;

pub fn compile(path: PathBuf, source: String) -> Result<parity_wasm::elements::Module, ArgonError> {
    let mut database = Database::new();

    let path = AbsolutePath::expand(path)?;

    database.add(path.clone(), source)?;

    let mut compilation = Compilation::new(database.shared());

    let module = compilation.get(&path);

    let module = match module {
        GetResult::Value(value) => value,
        GetResult::SkipResult(SkipResult::Error(err)) => {
            return Err(err);
        }
        GetResult::SkipResult(SkipResult::None) => unimplemented!(),
    };

    Ok(module.clone_value())
}
