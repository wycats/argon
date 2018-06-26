use super::resolved;
use crate::{ast, FunctionModifiers, MathOperator, MathType, Spanned, Type, UnifyTable};
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

    crate fn annotate<T>(self, item: T) -> Annotated<T> {
        Annotated { ty: self, item }
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

#[derive(Debug, Eq, PartialEq, Clone)]
crate struct Annotated<T> {
    crate ty: InferType,
    crate item: T,
}

impl<T> std::ops::Deref for Annotated<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.item
    }
}

#[derive(Debug)]
crate struct Module<'input> {
    crate funcs: Vec<Function<'input>>,
}

impl Module<'input> {
    crate fn from(
        resolved::Module { funcs }: resolved::Module<'input>,
        vars: &mut UnifyTable,
    ) -> Module<'input> {
        let funcs = funcs
            .into_iter()
            .map(|func| Function::from(func, vars))
            .collect();

        Module { funcs }
    }
}

#[derive(Debug)]
crate struct Function<'input> {
    crate name: Spanned<&'input str>,
    crate params: Vec<Type>,
    crate symbols: Vec<Spanned<&'input str>>,
    crate ret: Type,
    crate body: Annotated<Block>,
    crate modifiers: FunctionModifiers,
}

impl Function<'input> {
    crate fn from(
        resolved::Function {
            name,
            params,
            symbols,
            ret,
            body,
            modifiers,
        }: resolved::Function<'input>,
        vars: &mut UnifyTable,
    ) -> Function<'input> {
        let body = {
            let env = TypeEnv::params(&params);
            Block::from(body, vars, &env)
        };

        Function {
            name,
            params,
            symbols,
            ret,
            body,
            modifiers,
        }
    }
}

#[derive(Debug)]
crate struct Block {
    crate expressions: Vec<Annotated<Expression>>,
}

impl Block {
    crate fn from(
        block: resolved::Block,
        vars: &mut UnifyTable,
        env: &TypeEnv,
    ) -> Annotated<Block> {
        let expressions = block
            .expressions
            .into_iter()
            .map(|e| e.annotate(vars, &env))
            .collect();

        Annotated {
            item: Block { expressions },
            ty: vars.fresh(),
        }
    }
}

impl Block {
    crate fn last_ty(&self) -> InferType {
        match self.expressions.last() {
            None => InferType::Resolved(Type::Void),
            Some(e) => e.ty.clone(),
        }
    }
}

#[derive(Debug)]
crate enum Expression {
    Const(ast::ConstExpression),
    VariableAccess(u32),

    #[allow(unused)]
    Apply(Box<Annotated<Expression>>, Vec<Annotated<Expression>>),
    Binary {
        operator: MathOperator,
        lhs: Box<Annotated<Expression>>,
        rhs: Box<Annotated<Expression>>,
    },
}

impl Expression {
    #[cfg(test)]
    crate fn integer(value: i32) -> Expression {
        Expression::Const(ast::ConstExpression::Integer(value))
    }

    #[cfg(test)]
    crate fn bool(value: bool) -> Expression {
        Expression::Const(ast::ConstExpression::Bool(value))
    }

    crate fn annotate(self, ty: InferType) -> Annotated<Expression> {
        Annotated { item: self, ty }
    }
}

#[cfg(test)]
impl Annotated<Expression> {
    crate fn integer(ty: InferType, term: i32) -> Annotated<Expression> {
        Annotated {
            ty,
            item: Expression::integer(term),
        }
    }

    crate fn bool(ty: InferType, term: bool) -> Annotated<Expression> {
        Annotated {
            ty,
            item: Expression::bool(term),
        }
    }

    crate fn var(ty: InferType, term: u32) -> Annotated<Expression> {
        Annotated {
            ty,
            item: Expression::VariableAccess(term),
        }
    }

    crate fn apply(
        ty: InferType,
        func: Annotated<Expression>,
        args: Vec<Annotated<Expression>>,
    ) -> Annotated<Expression> {
        Annotated {
            ty,
            item: Expression::Apply(box func, args),
        }
    }
}
