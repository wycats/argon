mod tokenize;

pub use self::tokenize::Lexer;

use crate::ir::Spanned;
use nan_preserving_float::F64;
use std::borrow::Cow;
use std::fmt;

#[derive(Debug)]
pub struct LexicalError {}

#[derive(Debug, Clone, PartialEq)]
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
    Id(Cow<'input, str>),
    Int(i32),
    Float(F64),
    WS(Cow<'input, str>),
}

impl Tok<'input> {
    pub fn into_owned(&self) -> Tok<'static> {
        use self::Tok::*;

        match self {
            Id(id) => Id(Cow::Owned(id.clone().into_owned())),
            WS(ws) => WS(Cow::Owned(ws.clone().into_owned())),
            Export => Export,
            Def => Def,
            Arrow => Arrow,
            OpenParen => OpenParen,
            CloseParen => CloseParen,
            OpenBrace => OpenBrace,
            CloseBrace => CloseBrace,
            Colon => Colon,
            Comma => Comma,
            I32 => I32,
            I64 => I64,
            U32 => U32,
            U64 => U64,
            F32 => F32,
            F64 => F64,
            Add => Add,
            Sub => Sub,
            Mul => Mul,
            Div => Div,
            Int(int) => Int(*int),
            Float(float) => Float(*float),
        }
    }
}

impl Spanned<Tok<'input>> {
    pub fn into_owned(&self) -> Spanned<Tok<'static>> {
        Spanned {
            node: self.node.into_owned(),
            span: self.span.clone(),
        }
    }
}

impl fmt::Display for Tok<'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
