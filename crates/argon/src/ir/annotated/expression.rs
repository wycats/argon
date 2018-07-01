use super::types::InferType;
use super::Annotated;
use crate::infer::{Constraint, Constraints};
use crate::ir::ast;
use crate::ir::Spanned;
use crate::MathOperator;

#[derive(Debug)]
crate enum Expression {
    Const(ast::ConstExpression),
    VariableAccess(u32),

    #[allow(unused)]
    Apply(Box<Annotated<Expression>>, Vec<Annotated<Expression>>),
    Binary {
        operator: Spanned<MathOperator>,
        lhs: Box<Annotated<Expression>>,
        rhs: Box<Annotated<Expression>>,
    },
}

impl Expression {
    crate fn annotate(self, ty: InferType) -> Annotated<Expression> {
        Annotated { item: self, ty }
    }
}

impl Annotated<Expression> {
    crate fn constraints(&self) -> Constraints {
        let Annotated { ty, item } = self;

        match item {
            Expression::Apply(function, args) => {
                let mut arg_constraints = Constraints::empty();

                for arg in args {
                    arg_constraints += arg.constraints();
                }

                let args = args.iter().map(|a| a.ty.clone()).collect();

                function.constraints() + arg_constraints
                    + Constraints(Constraint(
                        function.ty.clone(),
                        InferType::variable_function(args, ty.clone()),
                    ))
            }
            Expression::Const(constant) => match constant {
                ast::ConstExpression::Bool(..) => {
                    Constraints(Constraint::new(ty.clone(), InferType::bool()))
                }

                ast::ConstExpression::Integer(..) => {
                    Constraints(Constraint::new(ty.clone(), InferType::integer()))
                }

                ast::ConstExpression::Float(..) => {
                    Constraints(Constraint::new(ty.clone(), InferType::float()))
                }
            },
            Expression::VariableAccess(_) => Constraints::empty(),
            Expression::Binary {
                operator: _,
                lhs: box lhs,
                rhs: box rhs,
            } => {
                lhs.constraints()
                    + rhs.constraints()
                    + Constraints(Constraint(ty.clone(), lhs.ty.clone()))
                    + Constraints(Constraint(ty.clone(), rhs.ty.clone()))
            }
        }
    }
}
