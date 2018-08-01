mod tokenize;

use crate::prelude::*;

pub use self::tokenize::Lexer;
use crate::ir::pos::{Spanned, SpannedItem};

#[derive(Debug)]
pub struct LexicalError {}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IdentifierId(u64);

impl IdentifierId {
    crate fn from_str(string: &str) -> IdentifierId {
        IdentifierId(seahash::hash(string.as_bytes()))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tok {
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
    Id(IdentifierId),
    Int(i32),
    Float(F64),
    WS,
}

impl Tok {
    crate fn id(s: &str) -> Tok {
        let id = IdentifierId::from_str(s);
        Tok::Id(id)
    }
}

pub type Token = Spanned<Tok>;

impl Token {
    crate fn to_spanned_i32(self) -> Spanned<i32> {
        use self::Tok::*;

        match self {
            Spanned { node: Int(int), .. } => int.copy_span(&self),
            _ => panic!("Expected Tok::Int, found {:?}",),
        }
    }

    crate fn to_spanned_f64(self) -> Spanned<F64> {
        use self::Tok::*;

        match self {
            Spanned {
                node: Float(float), ..
            } => float.copy_span(&self),
            _ => panic!("Expected Tok::Float, found {:?}",),
        }
    }

    crate fn to_ident(&self) -> IdentifierId {
        use self::Tok::*;

        match self {
            Spanned { node: Id(id), .. } => *id,
            _ => panic!("Expected Tok::Id, found {:?}",),
        }
    }
}

impl fmt::Display for Tok {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
