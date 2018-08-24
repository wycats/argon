use super::constraint::{Constraints, Why};
use super::substitution::Substitution;
use crate::errors::compile_error::UnifyError;
use crate::ir::{InferType, TypeVar};
use crate::pos::Spanned;
use crate::CompileError;
use ena::unify::InPlaceUnificationTable;
use log::*;
use std::collections::BTreeSet;

crate use self::table::UnifyTable;

crate mod table;

struct Unify {
    crate table: InPlaceUnificationTable<TypeVar>,
    crate constraints: Constraints,
    crate keys: BTreeSet<TypeVar>,
}

impl Unify {
    fn unify(self) -> Result<Substitution, CompileError> {
        let Unify {
            mut table,
            constraints,
            keys,
        } = self;

        {
            let mut unify = UnifyOne { table: &mut table };

            for constraint in &constraints {
                unify.constrain(&constraint.left, &constraint.right, constraint.why)?;
            }
        }

        let mut substitution = Substitution::empty();

        for key in &keys {
            let value = table.probe_value(*key);
            substitution.set(*key, value)
        }

        Ok(substitution)
    }
}

struct UnifyOne<'unify> {
    table: &'unify mut InPlaceUnificationTable<TypeVar>,
}

impl UnifyOne<'unify> {
    fn constrain(
        &mut self,
        left: &InferType,
        right: &InferType,
        why: Why,
    ) -> Result<(), CompileError> {
        trace!(target: "argon::unify::one", "+constraint {:?} {:?}", left, right);

        trace!(target: "argon::unify", "Constraining {:#?} + {:#?}", left, right);

        match (left, right) {
            (
                InferType::Resolved(Spanned { node: left, .. }),
                InferType::Resolved(Spanned { node: right, .. }),
            )
                if left == right => {}

            (InferType::Resolved(..), InferType::Resolved(..)) => {
                return Err(CompileError::Unimplemented)
            }

            (InferType::Constrained(c), InferType::Resolved(r)) => {
                if !c.node.unifies_ty(&r.node) {
                    println!("Failed to unify {:?} + {:?}", c, r);

                    return Err(CompileError::UnifyError(UnifyError {
                        left: InferType::Constrained(c.clone()),
                        right: InferType::Resolved(r.clone()),
                        why,
                    }));
                };
            }

            (r @ InferType::Resolved(..), c @ InferType::Constrained(..)) => {
                self.constrain(c, r, why)?
            }

            (InferType::Constrained(..), InferType::Constrained(..)) => {
                // float literals and int literals unify, but eventually we need a better
                // intersection system
            }

            (InferType::Variable(var), value @ InferType::Resolved(..)) => {
                self.table.unify_var_value(*var, value.clone()).map_err(
                    |UnifyError { left, right, .. }| {
                        CompileError::UnifyError(UnifyError { left, right, why })
                    },
                )?;
                self.recurse_left(*var, value, why)?;
            }

            (value @ InferType::Resolved(..), var @ InferType::Variable(..)) => {
                self.constrain(var, value, why)?;
            }

            (InferType::Variable(left), InferType::Variable(right)) => {
                self.table.unify_var_var(*left, *right).map_err(
                    |UnifyError { left, right, .. }| {
                        CompileError::UnifyError(UnifyError { left, right, why })
                    },
                )?;
                self.recurse_both(*left, *right, why)?;
            }

            (InferType::Variable(left), constrained @ InferType::Constrained(..)) => {
                self.recurse_left(*left, constrained, why)?;
            }

            (constrained @ InferType::Constrained(..), var @ InferType::Variable(..)) => {
                self.constrain(var, constrained, why)?;
            }

            (
                InferType::VariableFunction(lparams, lret),
                InferType::VariableFunction(rparams, rret),
            ) => {
                for (left, right) in lparams.iter().zip(rparams) {
                    self.constrain(left, right, why)?;
                }

                self.constrain(lret, rret, why)?;
            }

            (InferType::Variable(left), f @ InferType::VariableFunction(..)) => {
                self.recurse_left(*left, f, why)?;
            }

            (f @ InferType::VariableFunction(..), v @ InferType::Variable(..)) => {
                self.constrain(v, f, why)?;
            }

            (InferType::Variable(left), f @ InferType::Function(..)) => {
                self.recurse_left(*left, f, why)?;
            }

            (f @ InferType::Function(..), v @ InferType::Variable(..)) => {
                self.constrain(v, f, why)?;
            }

            (left, right) => unimplemented!("unifying constraints {:?} and {:?}", left, right),
        };

        trace!(target: "argon::unify", "-constraint table={:#?}", self.table);

        Ok(())
    }

    fn recurse_left(&mut self, var: TypeVar, ty: &InferType, why: Why) -> Result<(), CompileError> {
        let probed_left = self.table.probe_value(var);

        if InferType::Variable(var) == probed_left {
            return Ok(());
        }

        self.constrain(&probed_left, ty, why)
    }

    fn recurse_both(
        &mut self,
        left: TypeVar,
        right: TypeVar,
        why: Why,
    ) -> Result<(), CompileError> {
        let probed_left = self.table.probe_value(left);
        let probed_right = self.table.probe_value(right);

        if InferType::Variable(left) == probed_left && InferType::Variable(right) == probed_right {
            return Ok(());
        }

        self.constrain(&probed_left, &probed_right, why)
    }
}

#[allow(unused)]
fn occurs(type_var: TypeVar, ty: &InferType) -> bool {
    match ty {
        InferType::Function(params, ret) => {
            params
                .iter()
                .any(|ty| occurs(type_var, &InferType::Resolved(ty.clone())))
                || occurs(type_var, &InferType::Resolved(ret.clone()))
        }

        InferType::Variable(type_var2) => type_var == *type_var2,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::Substitution;
    use super::UnifyTable;
    use crate::infer::constraint::{Constraint, Constraints};
    use crate::ir::InferType;
    use crate::ir::Spanned;

    fn types() -> UnifyTable {
        UnifyTable::new()
    }

    #[test]
    fn unifies_two_ints() {
        crate::init_logger();

        let substitution = types().unify(Constraints(Constraint::double(
            InferType::i32(),
            InferType::i32(),
        )));

        assert_eq!(substitution, Ok(Substitution::empty()));
    }

    #[test]
    fn unifies_two_bools() {
        crate::init_logger();

        let substitution = types().unify(Constraints(Constraint::double(
            InferType::bool(),
            InferType::bool(),
        )));

        assert_eq!(substitution, Ok(Substitution::empty()));
    }

    #[test]
    fn unifies_two_functions() {
        crate::init_logger();

        let substitution = types().unify(Constraints(Constraint::double(
            InferType::variable_function(vec![InferType::bool()], InferType::bool()),
            InferType::variable_function(vec![InferType::bool()], InferType::bool()),
        )));

        assert_eq!(substitution, Ok(Substitution::empty()));
    }

    #[test]
    fn unifies_variables_with_non_variables() {
        crate::init_logger();

        let mut types = types();

        let x = types.synthetic();

        let substitution =
            types.unify(Constraints(Constraint::double(x.clone(), InferType::i32())));
        let expected = Substitution::from(&[(0, InferType::i32())]);

        assert_eq!(substitution, Ok(expected));
    }

    #[test]
    fn unifies_integer_literal() {
        crate::init_logger();

        let mut types = types();

        let t1 = types.synthetic();
        let t2 = types.synthetic();

        // def add(x: i64) -> i64 { x + 50 }

        let constraints = Constraint::double(t1.clone(), t2.clone())
            + Constraint::double(t1.clone(), InferType::i64())
            + Constraint::double(t2.clone(), InferType::integer(&Spanned::synthetic("test")));

        let substitution = types.unify(constraints);
        let expected = Substitution::from(&[(1, InferType::i64()), (0, InferType::i64())]);

        assert_eq!(substitution, Ok(expected));
    }

    #[test]
    fn unifies_variables_in_variable_functions() {
        crate::init_logger();

        let mut types = types();

        let t1 = types.synthetic();
        let t2 = types.synthetic();

        let substitution = types.unify(Constraints(Constraint::double(
            InferType::variable_function(vec![t1.clone()], InferType::bool()),
            InferType::variable_function(vec![InferType::i32()], t2.clone()),
        )));

        let expected = Substitution::from(&[(0, InferType::i32()), (1, InferType::bool())]);

        assert_eq!(substitution, Ok(expected));
    }

    #[test]
    fn unifies_multiple_variables_and_literals() {
        crate::init_logger();

        let mut types = types();

        let t0 = types.synthetic();
        let t1 = types.synthetic();
        let t2 = types.synthetic();
        let t3 = types.synthetic();

        let constraints = Constraint::double(t0.clone(), t1.clone())
            + Constraint::double(t0.clone(), t2.clone())
            + Constraint::double(t0.clone(), t3.clone())
            + Constraint::double(t1.clone(), InferType::f64())
            + Constraint::double(t2.clone(), InferType::float(&Spanned::synthetic("test")));

        let substitution = types.unify(constraints).unwrap();

        assert_eq!(substitution[0], InferType::f64());
        assert_eq!(substitution[1], InferType::f64());
        assert_eq!(substitution[2], InferType::f64());
        assert_eq!(substitution[3], InferType::f64());
    }
}
