use crate::ir::{Spanned, SpannedItem};
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
    crate params: &'input [Spanned<Type>],
}

impl TypeEnv<'input> {
    crate fn params(params: &'input [Spanned<Type>]) -> TypeEnv<'input> {
        TypeEnv { params }
    }

    crate fn get_local(&self, local: usize) -> Spanned<Type> {
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

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum InferType {
    Resolved(Spanned<Type>),
    Constrained(ConstrainedType),
    Function(Vec<Spanned<Type>>, Spanned<Type>),
    VariableFunction(Vec<InferType>, Box<InferType>),
    Variable(TypeVar),
}

impl fmt::Display for InferType {
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
    crate fn is_same_type(&self, other: &InferType) -> bool {
        match (self, other) {
            (
                InferType::Resolved(Spanned { node: lhs, .. }),
                InferType::Resolved(Spanned { node: rhs, .. }),
            ) => lhs == rhs,
            _ => false,
        }
    }

    crate fn into_type(self) -> Spanned<Type> {
        match self {
            InferType::Resolved(ty) => ty,
            other => panic!("Cannot convert a {:?} into a Type", other),
        }
    }

    crate fn as_math(&self) -> MathType {
        match self {
            InferType::Resolved(Spanned {
                node: Type::Math(ty),
                ..
            }) => *ty,
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
        InferType::Resolved(Type::bool().synthetic("test"))
    }
}

#[cfg(test)]
impl InferType {
    crate fn i32() -> InferType {
        InferType::Resolved(Type::i32().synthetic("test"))
    }

    crate fn i64() -> InferType {
        InferType::Resolved(Type::i64().synthetic("test"))
    }

    crate fn f64() -> InferType {
        InferType::Resolved(Type::f64().synthetic("test"))
    }
}
