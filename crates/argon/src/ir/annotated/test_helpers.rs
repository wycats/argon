use super::types::InferType;
use super::{Annotated, Expression};
use crate::ir::ast;
use crate::ir::pos::SpannedItem;

impl Expression {
    crate fn integer(value: i32) -> Expression {
        Expression::Const(ast::ConstExpression::Integer(value.synthetic("test")))
    }

    crate fn bool(value: bool) -> Expression {
        Expression::Const(ast::ConstExpression::Bool(value.synthetic("test")))
    }
}

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
