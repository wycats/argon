#![feature(crate_visibility_modifier)]
#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(try_trait)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![allow(unused_extern_crates)]

pub mod compilation;
pub mod compile;
pub mod database;
pub mod debuggable;
crate mod errors;
crate mod infer;
pub mod ir;
pub mod lexer;
pub mod parser;
crate mod prelude;

#[allow(warnings)]
pub mod grammar;

pub use self::compilation::{Compilation, Database, SharedDatabase};
pub use self::compile::compile as compile_module;
pub use self::compile::*;
pub use self::database::absolute_path::AbsolutePath;
pub use self::database::{GetResult, SkipResult};
pub use self::errors::compile_error::{CompileError, ToDiagnostic};
pub use self::errors::ArgonError;
pub use self::grammar::ModuleParser;
pub use self::ir::*;
pub use self::parser::LalrpopParseError;

crate use self::infer::unify::UnifyTable;

#[cfg(test)]
crate fn init_logger() {
    #![allow(unused_must_use)]
    pretty_env_logger::try_init();
}
