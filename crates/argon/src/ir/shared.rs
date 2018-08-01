use crate::prelude::*;

use crate::compile::math::MathType;
use crate::ir::pos::{Spanned, SpannedItem};
use derive_new::*;

#[derive(PartialEq, Copy, Clone)]
pub enum ConstExpression {
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    F32(F32),
    F64(F64),
}

impl fmt::Debug for ConstExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

#[derive(PartialEq, Eq, Ord, PartialOrd, Clone, Hash)]
pub enum Type {
    Math(MathType),
    Bool,
    Function(Box<FunctionType>),
    Apply(Box<FunctionType>, Vec<Type>),
    Void,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Hash)]
pub struct FunctionType {
    pub params: Vec<Type>,
    pub ret: Type,
}

#[allow(non_snake_case)]
pub fn FunctionType(params: Vec<Type>, ret: Type) -> FunctionType {
    FunctionType { params, ret }
}

impl Type {
    pub fn function(params: Vec<Type>, ret: Type) -> Type {
        Type::Function(box FunctionType { params, ret })
    }

    pub fn bool() -> Type {
        Type::Bool
    }

    pub fn is_integer(&self) -> bool {
        match self {
            Type::Math(math) => match math {
                MathType::I32 | MathType::I64 | MathType::U32 | MathType::U64 => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            Type::Math(math) => match math {
                MathType::F32 | MathType::F64 => true,
                _ => false,
            },
            _ => false,
        }
    }

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

    pub fn void() -> Spanned<Type> {
        Type::Void.synthetic("void")
    }
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Math(ty) => write!(f, "{:?}", ty),
            Type::Function(box FunctionType { params, ret }) => write!(
                f,
                "({}) -> {:?}",
                params.iter().map(|p| format!("{:?}", p)).join(", "),
                ret
            ),
            Type::Bool => write!(f, "boolean"),
            Type::Apply(box function, params) => write!(
                f,
                "(apply fn {:?} with {})",
                Type::Function(box function.clone()),
                params.iter().map(|p| format!("{:?}", p)).join(", ")
            ),
            Type::Void => write!(f, "void"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, new)]
pub struct FunctionModifiers {
    #[new(default)]
    pub export: bool,
}
