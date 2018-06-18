use crate::compile::math::{MathOperator, MathType};
use std::fmt;

#[derive(Debug)]
pub enum TypeError {
    MismatchedBinary(MathOperator, Type, Type),
}

#[derive(PartialEq, Copy, Clone)]
pub enum ConstExpression {
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
}

impl fmt::Debug for ConstExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConstExpression::I32(int) => write!(f, "{:?}", *int),
            ConstExpression::I64(int) => write!(f, "{:?}", *int),
            ConstExpression::U32(int) => write!(f, "{:?}", *int),
            ConstExpression::U64(int) => write!(f, "{:?}", *int),
            ConstExpression::F32(float) => write!(f, "{:?}", *float),
            ConstExpression::F64(float) => write!(f, "{:?}", *float),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Type {
    Math(MathType),
    Void,
}

impl Type {
    pub fn i32() -> Type {
        Type::Math(MathType::I32)
    }

    pub fn i64() -> Type {
        Type::Math(MathType::I64)
    }

    pub fn u32() -> Type {
        Type::Math(MathType::U32)
    }

    pub fn u64() -> Type {
        Type::Math(MathType::U64)
    }

    pub fn f32() -> Type {
        Type::Math(MathType::F32)
    }

    pub fn f64() -> Type {
        Type::Math(MathType::F64)
    }
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Math(ty) => write!(f, "{:?}", ty),
            Type::Void => write!(f, "void"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, new)]
pub struct FunctionModifiers {
    #[new(default)]
    pub export: bool,
}
