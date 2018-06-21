use super::constraint::{Constraint, Constraints};
use crate::annotated::TypeVar;
use crate::ir::InferType;
use crate::{CompileError, FunctionType, Type};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq)]
crate struct Substitution {
    solutions: HashMap<TypeVar, InferType>,
}

impl Substitution {
    fn new(solutions: HashMap<TypeVar, InferType>) -> Substitution {
        Substitution { solutions }
    }

    fn empty() -> Substitution {
        Substitution {
            solutions: HashMap::new(),
        }
    }

    fn from_pair(type_var: TypeVar, ty: InferType) -> Substitution {
        let mut solutions = HashMap::new();
        solutions.insert(type_var, ty);

        Substitution { solutions }
    }

    fn apply(&self, constraint: &Constraint) -> Constraint {
        Constraint(
            self.apply_type(&constraint.left),
            self.apply_type(&constraint.right),
        )
    }

    fn apply_type(&self, ty: &InferType) -> InferType {
        let mut result = ty;

        for (type_var, solution_type) in &self.solutions {
            result = self.substitute(result, type_var, solution_type);
        }

        result.clone()
    }

    fn substitute(
        &self,
        ty: &'apply InferType,
        type_var: &TypeVar,
        replacement: &'apply InferType,
    ) -> &'apply InferType {
        match ty {
            InferType::VariableFunction(params, ret) => unimplemented!(),

            InferType::Variable(type_var2) if type_var == type_var2 => replacement,

            _ => ty,
        }
    }

    fn compose(&self, other: Substitution) -> Substitution {
        let mut out = HashMap::new();

        for (type_var, ty) in &self.solutions {
            out.insert(*type_var, other.apply_type(ty));
        }

        out.extend(other.solutions);

        Substitution::new(out)
    }
}

crate fn unify(constraints: Constraints) -> Result<Substitution, CompileError> {
    if constraints.is_empty() {
        return Ok(Substitution::empty());
    }

    let (head, tail) = constraints.take_head();

    let substitution = unify_one(head)?;
    let substituted_tail = unify(tail)?;

    Ok(substitution.compose(substituted_tail))
}

fn unify_one(constraint: Constraint) -> Result<Substitution, CompileError> {
    match (constraint.left, constraint.right) {
        (InferType::Resolved(left), InferType::Resolved(right)) => unify_resolved(left, right),
        (
            InferType::VariableFunction(lparams, box lret),
            InferType::VariableFunction(rparams, box rret),
        ) => {
            let mut constraints = Constraints::empty();

            for (left, right) in lparams.into_iter().zip(rparams) {
                constraints += Constraint(left, right);
            }

            constraints += Constraint(lret, rret);

            unify(constraints)
        }
        (InferType::Variable(type_var), ty) => unify_var(type_var, &ty),
        (ty, InferType::Variable(type_var)) => unify_var(type_var, &ty),
        _ => return Err(CompileError::Unimplemented),
    }
}

fn unify_resolved(left: Type, right: Type) -> Result<Substitution, CompileError> {
    match (left, right) {
        (Type::Math(left), Type::Math(right)) if left == right => Ok(Substitution::empty()),
        (Type::Bool, Type::Bool) => Ok(Substitution::empty()),
        (
            Type::Function(box FunctionType {
                params: lparams,
                ret: lret,
            }),
            Type::Function(box FunctionType {
                params: rparams,
                ret: rret,
            }),
        ) => {
            let types = lparams.iter().zip(rparams);

            let mut constraints = Constraints::empty();

            for (left, right) in types {
                constraints += Constraint(
                    InferType::Resolved(left.clone()),
                    InferType::Resolved(right.clone()),
                );
            }

            constraints += Constraint(InferType::Resolved(lret), InferType::Resolved(rret));

            unify(constraints)
        }

        (Type::Void, Type::Void) => Ok(Substitution::empty()),

        _ => Err(CompileError::Unimplemented),
    }
}

fn unify_var(type_var: TypeVar, ty: &InferType) -> Result<Substitution, CompileError> {
    let substitution = match ty {
        InferType::Variable(type_var2) if type_var == *type_var2 => Substitution::empty(),
        InferType::Variable(..) => Substitution::from_pair(type_var, ty.clone()),
        ty if occurs(type_var, ty) => return Err(CompileError::Unimplemented),
        ty => Substitution::from_pair(type_var, ty.clone()),
    };

    Ok(substitution)
}

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
    use super::{unify, Substitution};
    use crate::infer::constraint::{Constraint, Constraints};
    use crate::ir::annotated::{TypeVar, TypeVars};
    use crate::ir::InferType;
    use std::collections::HashMap;

    fn types() -> TypeVars {
        TypeVars::new()
    }

    #[test]
    fn unifies_two_ints() {
        let substitution = unify(Constraints(Constraint(InferType::i32(), InferType::i32())));

        assert_eq!(substitution, Ok(Substitution::empty()));
    }

    #[test]
    fn unifies_two_bools() {
        let substitution = unify(Constraints(Constraint(
            InferType::bool(),
            InferType::bool(),
        )));

        assert_eq!(substitution, Ok(Substitution::empty()));
    }

    #[test]
    fn unifies_two_functions() {
        let substitution = unify(Constraints(Constraint(
            InferType::variable_function(vec![InferType::bool()], InferType::bool()),
            InferType::variable_function(vec![InferType::bool()], InferType::bool()),
        )));

        assert_eq!(substitution, Ok(Substitution::empty()));
    }

    #[test]
    fn unifies_two_variables() {
        let x = InferType::var(1);
        let y = InferType::var(2);

        let substitution = unify(Constraints(Constraint(x.clone(), y.clone())));
        let expected = Substitution::from_pair(TypeVar::new(1), InferType::var(2));

        assert_eq!(substitution, Ok(expected));
    }

    #[test]
    fn unifies_variables_with_non_variables() {
        let substitution = unify(Constraints(Constraint(InferType::var(1), InferType::i32())));
        let expected = Substitution::from_pair(TypeVar::new(1), InferType::i32());

        assert_eq!(substitution, Ok(expected));
    }

    #[test]
    fn unifies_variables_in_functions() {
        let substitution = unify(Constraints(Constraint(
            InferType::variable_function(vec![InferType::var(1)], InferType::bool()),
            InferType::variable_function(vec![InferType::i32()], InferType::var(2)),
        )));

        let mut expected = HashMap::new();
        expected.insert(TypeVar::new(1), InferType::i32());
        expected.insert(TypeVar::new(2), InferType::bool());
        let expected = Substitution::new(expected);

        assert_eq!(substitution, Ok(expected));
    }
}
