use super::constraint::Constraints;
use super::substitution::Substitution;
use crate::ir::{InferType, TypeVar};
use crate::CompileError;
use ena::unify::InPlaceUnificationTable;
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
                unify.constrain(&constraint.left, &constraint.right)?;
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
    fn constrain(&mut self, left: &InferType, right: &InferType) -> Result<(), CompileError> {
        trace!(target: "wasm::unify::one", "+constraint {:?} {:?}", left, right);

        match (left, right) {
            (left @ InferType::Resolved(..), right @ InferType::Resolved(..)) if left == right => {}

            (_left @ InferType::Resolved(..), _right @ InferType::Resolved(..)) => {
                return Err(CompileError::Unimplemented)
            }

            (InferType::Constrained(c), InferType::Resolved(r)) => {
                if !c.unifies_ty(r) {
                    return Err(CompileError::UnifyError(
                        InferType::Constrained(c.clone()),
                        InferType::Resolved(r.clone()),
                    ));
                };
            }

            (r @ InferType::Resolved(..), c @ InferType::Constrained(..)) => self.constrain(c, r)?,

            (InferType::Constrained(..), InferType::Constrained(..)) => {
                // float literals and int literals unify, but eventually we need a better
                // intersection system
            }

            (InferType::Variable(var), value @ InferType::Resolved(..)) => {
                self.table.unify_var_value(*var, value.clone())?;
                self.recurse_left(*var, value)?;
            }

            (value @ InferType::Resolved(..), var @ InferType::Variable(..)) => {
                self.constrain(var, value)?;
            }

            (InferType::Variable(left), InferType::Variable(right)) => {
                self.table.unify_var_var(*left, *right)?;
                self.recurse_both(*left, *right)?;
            }

            (InferType::Variable(left), constrained @ InferType::Constrained(..)) => {
                self.recurse_left(*left, constrained)?;
            }

            (constrained @ InferType::Constrained(..), var @ InferType::Variable(..)) => {
                self.constrain(var, constrained)?;
            }

            (
                InferType::VariableFunction(lparams, lret),
                InferType::VariableFunction(rparams, rret),
            ) => {
                for (left, right) in lparams.iter().zip(rparams) {
                    self.constrain(left, right)?;
                }

                self.constrain(lret, rret)?;
            }

            (InferType::Variable(left), f @ InferType::VariableFunction(..)) => {
                self.recurse_left(*left, f)?;
            }

            (f @ InferType::VariableFunction(..), v @ InferType::Variable(..)) => {
                self.constrain(v, f)?;
            }

            (InferType::Variable(left), f @ InferType::Function(..)) => {
                self.recurse_left(*left, f)?;
            }

            (f @ InferType::Function(..), v @ InferType::Variable(..)) => {
                self.constrain(v, f)?;
            }

            (left, right) => unimplemented!("unifying constraints {:?} and {:?}", left, right),
        };

        trace!(target: "wasm::unify", "-constraint table={:#?}", self.table);

        Ok(())
    }

    fn recurse_left(&mut self, var: TypeVar, ty: &InferType) -> Result<(), CompileError> {
        let probed_left = self.table.probe_value(var);

        if InferType::Variable(var) == probed_left {
            return Ok(());
        }

        self.constrain(&probed_left, ty)
    }

    fn recurse_both(&mut self, left: TypeVar, right: TypeVar) -> Result<(), CompileError> {
        let probed_left = self.table.probe_value(left);
        let probed_right = self.table.probe_value(right);

        if InferType::Variable(left) == probed_left && InferType::Variable(right) == probed_right {
            return Ok(());
        }

        self.constrain(&probed_left, &probed_right)
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

    fn types() -> UnifyTable {
        UnifyTable::new()
    }

    #[test]
    fn unifies_two_ints() {
        crate::init_logger();

        let substitution =
            types().unify(Constraints(Constraint(InferType::i32(), InferType::i32())));

        assert_eq!(substitution, Ok(Substitution::empty()));
    }

    #[test]
    fn unifies_two_bools() {
        crate::init_logger();

        let substitution = types().unify(Constraints(Constraint(
            InferType::bool(),
            InferType::bool(),
        )));

        assert_eq!(substitution, Ok(Substitution::empty()));
    }

    #[test]
    fn unifies_two_functions() {
        crate::init_logger();

        let substitution = types().unify(Constraints(Constraint(
            InferType::variable_function(vec![InferType::bool()], InferType::bool()),
            InferType::variable_function(vec![InferType::bool()], InferType::bool()),
        )));

        assert_eq!(substitution, Ok(Substitution::empty()));
    }

    #[test]
    fn unifies_variables_with_non_variables() {
        crate::init_logger();

        let mut types = types();

        let x = types.fresh();

        let substitution = types.unify(Constraints(Constraint(x.clone(), InferType::i32())));
        let expected = Substitution::from(&[(0, InferType::i32())]);

        assert_eq!(substitution, Ok(expected));
    }

    #[test]
    fn unifies_integer_literal() {
        crate::init_logger();

        let mut types = types();

        let t1 = types.fresh();
        let t2 = types.fresh();

        // def add(x: i64) -> i64 { x + 50 }

        let constraints = Constraint(t1.clone(), t2.clone())
            + Constraint(t1.clone(), InferType::i64())
            + Constraint(t2.clone(), InferType::integer());

        let substitution = types.unify(constraints);
        let expected = Substitution::from(&[(1, InferType::i64()), (0, InferType::i64())]);

        assert_eq!(substitution, Ok(expected));
    }

    #[test]
    fn unifies_variables_in_variable_functions() {
        crate::init_logger();

        let mut types = types();

        let t1 = types.fresh();
        let t2 = types.fresh();

        let substitution = types.unify(Constraints(Constraint(
            InferType::variable_function(vec![t1.clone()], InferType::bool()),
            InferType::variable_function(vec![InferType::i32()], t2.clone()),
        )));

        let expected = Substitution::from(&[(0, InferType::i32()), (1, InferType::bool())]);

        assert_eq!(substitution, Ok(expected));
    }
}
