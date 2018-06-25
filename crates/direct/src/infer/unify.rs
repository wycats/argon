use super::constraint::Constraints;
use super::substitution::Substitution;
use crate::annotated::{Annotated, TypeVar};
use crate::ir::InferType;
use crate::CompileError;
use ena::unify::{InPlaceUnificationTable, UnifyKey, UnifyValue};
use std::collections::BTreeSet;

impl UnifyKey for TypeVar {
    type Value = InferType;

    fn index(&self) -> u32 {
        self.var as u32
    }

    fn from_index(u: u32) -> TypeVar {
        TypeVar::new(u as usize)
    }

    fn tag() -> &'static str {
        "type"
    }
}

impl UnifyValue for InferType {
    type Error = CompileError;

    fn unify_values(a: &InferType, b: &InferType) -> Result<InferType, CompileError> {
        match (a, b) {
            (lhs @ InferType::Resolved(..), rhs @ InferType::Resolved(..)) if lhs == rhs => {
                Ok(lhs.clone())
            }

            (left @ InferType::Variable(..), InferType::Variable(..)) => Ok(left.clone()),

            (InferType::Variable(..), other) => Ok(other.clone()),
            (other, InferType::Variable(..)) => Ok(other.clone()),

            (InferType::Resolved(resolved), InferType::Constrained(constrained))
                if constrained.unifies(resolved) =>
            {
                Ok(InferType::Resolved(resolved.clone()))
            }

            (c @ InferType::Constrained(..), r @ InferType::Resolved(..)) => {
                UnifyValue::unify_values(r, c)
            }

            (InferType::VariableFunction(..), f @ InferType::Function(..)) => Ok(f.clone()),
            (f @ InferType::Function(..), InferType::VariableFunction(..)) => Ok(f.clone()),

            (
                InferType::VariableFunction(lparams, box lret),
                InferType::VariableFunction(rparams, box rret),
            ) => {
                let mut params = vec![];

                for (lparam, rparam) in lparams.iter().zip(rparams) {
                    params.push(UnifyValue::unify_values(lparam, rparam)?);
                }

                let ret = UnifyValue::unify_values(lret, rret)?;

                Ok(InferType::VariableFunction(params, box ret))
            }

            _ => Err(CompileError::UnifyError(a.clone(), b.clone())),
        }
    }
}

crate struct UnifyTable {
    crate table: InPlaceUnificationTable<TypeVar>,
    crate keys: BTreeSet<TypeVar>,
}

impl UnifyTable {
    crate fn new() -> UnifyTable {
        UnifyTable {
            table: InPlaceUnificationTable::new(),
            keys: BTreeSet::new(),
        }
    }

    crate fn fresh(&mut self) -> InferType {
        let next = self.table.len();
        trace!(target: "wasm::unify", "Generating <T{}>", next);
        let ty = InferType::Variable(TypeVar::new(next));
        let key = self.table.new_key(ty.clone());
        self.keys.insert(key);
        ty
    }

    crate fn annotate_fresh<T>(&mut self, item: T) -> Annotated<T> {
        let ty = self.fresh();
        Annotated { item, ty }
    }

    crate fn unify(&self, constraints: Constraints) -> Result<Substitution, CompileError> {
        trace!(target: "wasm::unify", "Unifying {:#?}", constraints);

        let UnifyTable { table, keys } = self;

        let unify = Unify {
            table: table.clone(),
            constraints,
            keys: keys.clone(),
        };

        unify.unify()
    }
}

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
                unify.constraint((&constraint.left, &constraint.right))?;
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
    fn constraint(&mut self, (left, right): (&InferType, &InferType)) -> Result<(), CompileError> {
        trace!(target: "wasm::unify::one", "+constraint {:?} {:?}", left, right);

        match (left, right) {
            (left @ InferType::Resolved(..), right @ InferType::Resolved(..)) if left == right => {}

            (_left @ InferType::Resolved(..), _right @ InferType::Resolved(..)) => {
                return Err(CompileError::Unimplemented)
            }

            (InferType::Variable(var), value @ InferType::Resolved(..)) => {
                self.table.unify_var_value(*var, value.clone())?;
            }

            (value @ InferType::Resolved(..), var @ InferType::Variable(..)) => {
                self.constraint((var, value))?;
            }

            (InferType::Variable(left), InferType::Variable(right)) => {
                self.table.unify_var_var(*left, *right)?;
            }

            (InferType::Variable(left), constrained @ InferType::Constrained(..)) => {
                self.table.unify_var_value(*left, constrained.clone())?;
            }

            (constrained @ InferType::Constrained(..), var @ InferType::Variable(..)) => {
                self.constraint((var, constrained))?;
            }

            (
                InferType::VariableFunction(lparams, lret),
                InferType::VariableFunction(rparams, rret),
            ) => {
                for (left, right) in lparams.iter().zip(rparams) {
                    self.constraint((left, right))?;
                }

                self.constraint((lret, rret))?;
            }

            (InferType::Variable(left), f @ InferType::VariableFunction(..)) => {
                self.table.unify_var_value(*left, f.clone())?;
            }

            (f @ InferType::VariableFunction(..), v @ InferType::Variable(..)) => {
                self.constraint((v, f))?;
            }

            (InferType::Variable(left), f @ InferType::Function(..)) => {
                self.table.unify_var_value(*left, f.clone())?;
            }

            (f @ InferType::Function(..), v @ InferType::Variable(..)) => {
                self.constraint((v, f))?;
            }

            (left, right) => unimplemented!("unifying constraints {:?} and {:?}", left, right),
        };

        trace!(target: "wasm::unify", "-constraint table={:#?}", self.table);

        Ok(())
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
    use super::{Substitution, UnifyTable};
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
