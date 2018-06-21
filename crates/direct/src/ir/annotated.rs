use super::resolved;
use crate::{ast, FunctionModifiers, FunctionType, MathOperator, Spanned, Type};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
crate struct TypeVar {
    var: usize,
}

impl TypeVar {
    crate fn new(var: usize) -> TypeVar {
        TypeVar { var }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
crate struct TypeVars {
    current: usize,
}

impl TypeVars {
    crate fn new() -> TypeVars {
        TypeVars { current: 0 }
    }

    crate fn fresh(&mut self) -> InferType {
        let current = self.current;
        self.current += 1;

        InferType::Variable(TypeVar::new(current))
    }

    crate fn annotate_fresh<T>(&mut self, item: T) -> Annotated<T> {
        Annotated {
            ty: self.fresh(),
            item,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
crate enum InferType {
    Resolved(Type),
    Function(Vec<Type>, Type),
    VariableFunction(Vec<InferType>, Box<InferType>),
    Variable(TypeVar),
}

impl InferType {
    crate fn annotate<T>(self, item: T) -> Annotated<T> {
        Annotated { ty: self, item }
    }

    crate fn function(params: Vec<Type>, ret: Type) -> InferType {
        InferType::Function(params, ret)
    }

    #[cfg(test)]
    crate fn var(var: usize) -> InferType {
        InferType::Variable(TypeVar { var })
    }

    crate fn variable_function(params: Vec<InferType>, ret: InferType) -> InferType {
        InferType::VariableFunction(params, box ret)
    }

    crate fn i32() -> InferType {
        InferType::Resolved(Type::i32())
    }

    crate fn f32() -> InferType {
        InferType::Resolved(Type::f32())
    }

    crate fn bool() -> InferType {
        InferType::Resolved(Type::bool())
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

impl<T> Annotated<T> {
    crate fn new(ty: InferType, item: T) -> Annotated<T> {
        Annotated { ty, item }
    }
}

#[derive(Debug)]
crate struct Module<'input> {
    crate funcs: Vec<Function<'input>>,
}

impl Module<'input> {
    crate fn from(
        resolved::Module { funcs }: resolved::Module<'input>,
        vars: &mut TypeVars,
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
        vars: &mut TypeVars,
    ) -> Function<'input> {
        let body = Block::from(body, vars);

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
    crate fn from(block: resolved::Block, vars: &mut TypeVars) -> Annotated<Block> {
        let expressions = block
            .expressions
            .into_iter()
            .map(|e| Expression::from(e, vars))
            .collect();

        vars.annotate_fresh(Block { expressions })
    }
}

#[derive(Debug)]
crate enum Expression {
    Const(ast::ConstExpression),
    VariableAccess(u32),
    Apply(Box<Annotated<Expression>>, Vec<Annotated<Expression>>),
    Binary {
        operator: MathOperator,
        lhs: Box<Annotated<Expression>>,
        rhs: Box<Annotated<Expression>>,
    },
}

impl Expression {
    #[cfg(test)]
    crate fn i32(value: i32) -> Expression {
        Expression::Const(ast::ConstExpression::Integer(value as i64))
    }

    #[cfg(test)]
    crate fn bool(value: bool) -> Expression {
        Expression::Const(ast::ConstExpression::Bool(value))
    }

    crate fn from(expr: resolved::Expression, vars: &mut TypeVars) -> Annotated<Expression> {
        match expr {
            resolved::Expression::Const(expr) => vars.annotate_fresh(Expression::Const(expr)),
            resolved::Expression::VariableAccess(id) => {
                vars.annotate_fresh(Expression::VariableAccess(id))
            }
            resolved::Expression::Binary {
                operator,
                box lhs,
                box rhs,
            } => {
                let t1 = vars.fresh();
                t1.annotate(Expression::Binary {
                    operator,
                    lhs: box Expression::from(lhs, vars),
                    rhs: box Expression::from(rhs, vars),
                })
            }
        }
    }
}

#[cfg(test)]
impl Annotated<Expression> {
    crate fn i32(ty: InferType, term: i32) -> Annotated<Expression> {
        Annotated {
            ty,
            item: Expression::i32(term),
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

    crate fn function(ty: InferType, params: Vec<Type>, ret: Type) -> Annotated<FunctionType> {
        Annotated {
            ty,
            item: FunctionType(params, ret),
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
