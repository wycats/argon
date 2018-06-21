crate use super::constraint_set::Constraints;
use crate::ir::annotated::{self, Annotated};
use crate::ir::InferType;
use crate::{ast, FunctionType, Type};
use std::collections::HashSet;

#[derive(Debug, Hash, Eq, PartialEq)]
crate struct Constraint {
    crate left: InferType,
    crate right: InferType,
}

impl Constraint {
    crate fn new(left: InferType, right: InferType) -> Constraint {
        Constraint { left, right }
    }
}

impl std::ops::Add for Constraint {
    type Output = Constraints;

    fn add(self, rhs: Constraint) -> Constraints {
        Constraints(self) + rhs
    }
}

crate fn Constraint(left: InferType, right: InferType) -> Constraint {
    Constraint { left, right }
}

trait Collect {
    fn collect(&self) -> Constraints;
}

impl Collect for Vec<Annotated<annotated::Expression>> {
    fn collect(&self) -> Constraints {
        let mut constraints = Constraints::empty();

        for expression in self {
            constraints += expression.collect();
        }

        constraints
    }
}

impl Collect for Annotated<annotated::Expression> {
    fn collect(&self) -> Constraints {
        let Annotated { ty, item } = self;

        match item {
            annotated::Expression::Apply(function, args) => {
                let mut arg_constraints = Constraints::empty();

                for arg in args {
                    arg_constraints += arg.collect();
                }

                let args = args.iter().map(|a| a.ty.clone()).collect();

                function.collect() + arg_constraints
                    + Constraints(Constraint(
                        function.ty.clone(),
                        InferType::fresh_function(args, ty.clone()),
                    ))
            }
            annotated::Expression::Const(constant) => match constant {
                ast::ConstExpression::Bool(..) => {
                    Constraints(Constraint::new(ty.clone(), InferType::bool()))
                }

                ast::ConstExpression::Integer(..) => {
                    Constraints(Constraint::new(ty.clone(), InferType::i32()))
                }

                ast::ConstExpression::Float(..) => {
                    Constraints(Constraint::new(ty.clone(), InferType::f32()))
                }
            },
            annotated::Expression::VariableAccess(_) => Constraints::empty(),
            annotated::Expression::Binary {
                operator,
                lhs: box lhs,
                rhs: box rhs,
            } => {
                lhs.collect()
                    + rhs.collect()
                    + Constraints(Constraint(ty.clone(), lhs.ty.clone()))
                    + Constraints(Constraint(ty.clone(), rhs.ty.clone()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Collect;
    use super::{Constraint, Constraints};
    use crate::ir::annotated::{Annotated, Expression, TypeVars};
    use crate::ir::InferType as Type;

    type Term = Annotated<Expression>;

    fn types() -> TypeVars {
        TypeVars::new()
    }

    #[test]
    fn constrains_int() {
        let mut types = types();

        let t1 = types.fresh();
        let term = Term::i32(t1.clone(), 1);

        assert_eq!(term.collect(), Constraints(Constraint(t1, Type::i32())))
    }

    #[test]
    fn constrains_bool() {
        let mut types = types();

        let t1 = types.fresh();
        let term = Term::bool(t1.clone(), true);

        assert_eq!(term.collect(), Constraints(Constraint(t1, Type::bool())))
    }

    #[test]
    fn constrains_var() {
        let mut types = types();

        let t1 = types.fresh();
        let term = Term::var(t1, 0);

        assert_eq!(term.collect(), Constraints::empty())
    }

    #[test]
    fn constrains_app() {
        let mut types = types();

        let t1 = types.fresh();
        let t2 = types.fresh();
        let t3 = types.fresh();

        let func = Term::var(t2.clone(), 0);
        let arg = Term::var(t3.clone(), 1);
        let application = Term::apply(t1.clone(), func, vec![arg]);

        let expected = Constraints::empty()
            + Constraint(
                t2.clone(),
                Type::fresh_function(vec![t3.clone()], t1.clone()),
            );

        assert_eq!(application.collect(), expected);
    }
}
