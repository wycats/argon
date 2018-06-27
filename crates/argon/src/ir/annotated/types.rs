use crate::{MathType, Type};
use itertools::Itertools;
use std::fmt;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TypeVar {
    crate var: usize,
}

impl TypeVar {
    crate fn new(var: usize) -> TypeVar {
        TypeVar { var }
    }
}

impl fmt::Debug for TypeVar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<T{}>", self.var)
    }
}

// The name resolution phase resolves names to offsets, which are looked
// up in the TypeEnv
crate struct TypeEnv<'input> {
    // When we get locals, this will need to be changed
    crate params: &'input [Type],
}

impl TypeEnv<'input> {
    crate fn params(params: &'input [Type]) -> TypeEnv<'input> {
        TypeEnv { params }
    }

    crate fn get_local(&self, local: usize) -> Type {
        self.params[local].clone()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ConstrainedType {
    Integer,
    Float,
}

impl ConstrainedType {
    crate fn unifies_ty(&self, other: &Type) -> bool {
        let ty = match other {
            Type::Math(math) => math,
            _ => return false,
        };

        match self {
            ConstrainedType::Integer => true,
            ConstrainedType::Float => match ty {
                MathType::F32 | MathType::F64 => true,
                _ => false,
            },
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum InferType {
    Resolved(Type),
    Constrained(ConstrainedType),
    Function(Vec<Type>, Type),
    VariableFunction(Vec<InferType>, Box<InferType>),
    Variable(TypeVar),
}

impl fmt::Debug for InferType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InferType::Resolved(ty) => write!(f, "{:?}", ty),
            InferType::Constrained(constrained) => write!(f, "{:?}", constrained),
            InferType::Function(params, ret) => write!(
                f,
                "({}) -> {:?}",
                params.iter().map(|p| format!("{:?}", p)).join(", "),
                ret
            ),
            InferType::VariableFunction(params, ret) => write!(
                f,
                "({}) -> {:?}",
                params.iter().map(|p| format!("{:?}", p)).join(", "),
                ret
            ),
            InferType::Variable(var) => write!(f, "{:?}", var),
        }
    }
}

impl InferType {
    crate fn into_type(self) -> Type {
        match self {
            InferType::Resolved(ty) => ty,
            other => panic!("Cannot convert a {:?} into a Type", other),
        }
    }

    crate fn as_math(&self) -> MathType {
        match self {
            InferType::Resolved(Type::Math(ty)) => *ty,
            other => panic!("Cannot convert a {:?} into a MathType", other),
        }
    }

    crate fn variable_function(params: Vec<InferType>, ret: InferType) -> InferType {
        InferType::VariableFunction(params, box ret)
    }

    crate fn integer() -> InferType {
        InferType::Constrained(ConstrainedType::Integer)
    }

    crate fn float() -> InferType {
        InferType::Constrained(ConstrainedType::Float)
    }

    crate fn bool() -> InferType {
        InferType::Resolved(Type::bool())
    }
}

#[cfg(test)]
impl InferType {
    crate fn i32() -> InferType {
        InferType::Resolved(Type::i32())
    }

    crate fn i64() -> InferType {
        InferType::Resolved(Type::i64())
    }
}
