use super::resolved;
use crate::{ast, FunctionModifiers, MathOperator, Spanned, Type};

#[derive(Debug, Eq, PartialEq)]
crate enum InferType {
    Resolved(Type),
    Fresh,
}

impl InferType {
    crate fn annotate<T>(self, item: T) -> Annotated<T> {
        Annotated { ty: self, item }
    }
}

#[derive(Debug, Eq, PartialEq)]
crate struct Annotated<T> {
    ty: InferType,
    item: T,
}

impl<T> Annotated<T> {
    crate fn new(ty: InferType, item: T) -> Annotated<T> {
        Annotated { ty, item }
    }

    fn fresh(item: T) -> Annotated<T> {
        Annotated {
            ty: InferType::Fresh,
            item,
        }
    }
}

#[derive(Debug)]
crate struct Module<'input> {
    crate funcs: Vec<Function<'input>>,
}

impl Module<'input> {
    crate fn from(resolved::Module { funcs }: resolved::Module<'input>) -> Module<'input> {
        let funcs = funcs.into_iter().map(Function::from).collect();

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
    ) -> Function<'input> {
        let body = Block::from(body);

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
    crate fn from(block: resolved::Block) -> Annotated<Block> {
        let expressions = block
            .expressions
            .into_iter()
            .map(Expression::from)
            .collect();

        Annotated::fresh(Block { expressions })
    }
}

#[derive(Debug)]
crate enum Expression {
    Const(ast::ConstExpression),
    VariableAccess(u32),
    Binary {
        operator: MathOperator,
        lhs: Box<Annotated<Expression>>,
        rhs: Box<Annotated<Expression>>,
    },
}

impl Expression {
    crate fn from(expr: resolved::Expression) -> Annotated<Expression> {
        match expr {
            resolved::Expression::Const(expr) => Annotated::fresh(Expression::Const(expr)),
            resolved::Expression::VariableAccess(id) => {
                Annotated::fresh(Expression::VariableAccess(id))
            }
            resolved::Expression::Binary {
                operator,
                box lhs,
                box rhs,
            } => Annotated::fresh(Expression::Binary {
                operator,
                lhs: box Expression::from(lhs),
                rhs: box Expression::from(rhs),
            }),
        }
    }
}
