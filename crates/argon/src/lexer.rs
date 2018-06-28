mod tokenize;

pub use self::tokenize::Lexer;

use nan_preserving_float::F64;
use std::fmt;

#[derive(Debug)]
pub struct LexicalError {}

#[derive(Debug, Copy, Clone)]
pub enum Tok<'input> {
    Export,
    Def,
    Arrow,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Colon,
    Comma,
    I32,
    I64,
    U32,
    U64,
    F32,
    F64,
    Add,
    Sub,
    Mul,
    Div,
    Id(&'input str),
    Int(i32),
    Float(F64),
    WS(&'input str),
}

impl fmt::Display for Tok<'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
