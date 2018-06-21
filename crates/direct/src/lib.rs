#![feature(crate_visibility_modifier)]
#![feature(box_patterns)]
#![feature(box_syntax)]
#![deny(rust_2018_idioms)]
#![allow(unused_extern_crates)]
#![allow(warnings)]

#[macro_use]
extern crate derive_new;

#[macro_use]
extern crate lalrpop_util;

pub mod compile;
crate mod infer;
pub mod ir;
pub mod parser;

pub mod test_helpers;

lalrpop_mod!(pub grammar);

pub use self::compile::*;
pub use self::grammar::{ModuleParser, Token};
pub use self::ir::*;

pub use self::test_helpers::AstBuilder;
