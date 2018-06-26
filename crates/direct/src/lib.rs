#![feature(crate_visibility_modifier)]
#![feature(box_patterns)]
#![feature(box_syntax)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![allow(unused_extern_crates)]

#[macro_use]
extern crate derive_new;

#[macro_use]
extern crate lalrpop_util;

#[macro_use]
extern crate log;

pub mod compile;
crate mod infer;
pub mod ir;
pub mod lexer;
pub mod parser;

pub mod test_helpers;

lalrpop_mod!(pub grammar);

pub use self::compile::*;
pub use self::grammar::ModuleParser;
pub use self::ir::*;
pub use self::parser::ParseError;

pub use self::test_helpers::AstBuilder;

crate use self::infer::unify::UnifyTable;

#[cfg(test)]
crate fn init_logger() {
    #![allow(unused_must_use)]
    pretty_env_logger::try_init();
}
