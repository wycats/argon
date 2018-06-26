mod tokenize;

pub use self::tokenize::Lexer;

use nan_preserving_float::F64;

#[derive(Debug)]
pub struct LexicalError {}

#[derive(Debug)]
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

impl Tok<'input> {
    crate fn to_int(&self) -> i32 {
        match self {
            Tok::Int(int) => *int,
            _ => unreachable!(),
        }
    }

    crate fn to_float(&self) -> F64 {
        match self {
            Tok::Float(float) => *float,
            _ => unreachable!(),
        }
    }
}
