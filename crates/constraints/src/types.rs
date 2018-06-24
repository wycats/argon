use crate::collections::TypeVarSet;

// Next steps: normalize constraints by resolving any Equal(Concrete) and propagating the result

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
pub struct ExternType(pub usize);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TypeVars {
    current: usize,
}

impl TypeVars {
    crate fn new() -> TypeVars {
        TypeVars { current: 0 }
    }

    crate fn fresh(&mut self) -> Type {
        let current = self.current;
        self.current += 1;

        Type::Var(TypeVar(current))
    }
}

pub fn types() -> TypeVars {
    TypeVars::new()
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
pub struct TypeVar(usize);

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
pub struct Function(Vec<Type>, Box<Type>);

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
pub enum Type {
    Var(TypeVar),
    Concrete(ExternType),
    Function(Function),
    Apply(Box<Type>, Vec<Type>),
}

impl Type {
    pub fn var(number: usize) -> Type {
        Type::Var(TypeVar(number))
    }

    pub fn ty(ty: ExternType) -> Type {
        Type::Concrete(ty)
    }

    pub fn func(params: impl Into<Vec<Type>>, ret: Type) -> Type {
        let func = Function(params.into(), box ret);
        Type::Function(func)
    }

    pub fn apply(func: Type, args: impl Into<Vec<Type>>) -> Type {
        Type::Apply(box func, args.into())
    }

    pub fn vars(&self) -> TypeVarSet {
        match self {
            Type::Var(var) => TypeVarSet::var(*var),
            Type::Concrete(..) => TypeVarSet::empty(),
            Type::Function(Function(args, box ret)) => {
                let mut vars = ret.vars();

                for item in args {
                    vars += item.vars();
                }

                vars
            }

            Type::Apply(box function_type, params) => {
                let mut vars = function_type.vars();

                for item in params {
                    vars += item.vars();
                }

                vars
            }
        }
    }
}
