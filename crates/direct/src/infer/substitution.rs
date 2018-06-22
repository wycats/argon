use super::constraint::{Constraint, Constraints};
use crate::annotated::{self, TypeVar};
use crate::ir::{typed, InferType};
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
crate struct Substitution {
    solutions: HashMap<TypeVar, InferType>,
}

impl Substitution {
    crate fn new(solutions: HashMap<TypeVar, InferType>) -> Substitution {
        Substitution { solutions }
    }

    crate fn empty() -> Substitution {
        Substitution {
            solutions: HashMap::new(),
        }
    }

    crate fn from_pair(type_var: TypeVar, ty: InferType) -> Substitution {
        let mut solutions = HashMap::new();
        solutions.insert(type_var, ty);

        Substitution { solutions }
    }

    crate fn apply(&self, constraint: &Constraint) -> Constraint {
        Constraint(
            self.apply_type(&constraint.left),
            self.apply_type(&constraint.right),
        )
    }

    crate fn apply_module(
        &self,
        annotated::Module { funcs }: annotated::Module<'input>,
    ) -> typed::TypedModule<'input> {
        typed::TypedModule {
            funcs: funcs
                .into_iter()
                .map(|func| self.apply_function(func))
                .collect(),
        }
    }

    crate fn apply_function(
        &self,
        annotated::Function {
            name,
            params,
            symbols,
            ret,
            body: annotated::Annotated { ty, item: body },
            modifiers,
        }: annotated::Function<'input>,
    ) -> typed::TypedFunction<'input> {
        let body = self.apply_block(body);

        typed::TypedFunction {
            name,
            params,
            symbols,
            ret,
            body,
            modifiers,
        }
    }

    crate fn apply_block(&self, block: annotated::Block) -> typed::TypedBlock {
        let expressions = block
            .expressions
            .into_iter()
            .map(|expr| {
                let ty = self.apply_type(&expr.ty);
                trace!(target: "wasm::applied", "before: {:?}, after: {:?}", expr.ty, ty);
                expr.into_typed_expression(&ty)
            })
            .collect();

        typed::TypedBlock { expressions }
    }

    crate fn apply_type(&self, ty: &InferType) -> InferType {
        let mut result = ty.clone();

        for (type_var, solution_type) in &self.solutions {
            result = self.substitute(&result, type_var, solution_type);
        }

        result.clone()
    }

    crate fn substitute(
        &self,
        ty: &'apply InferType,
        type_var: &TypeVar,
        replacement: &'apply InferType,
    ) -> InferType {
        match ty {
            InferType::VariableFunction(params, ret) => {
                let params = params.iter().map(|param| self.apply_type(param)).collect();
                let ret = self.apply_type(ret);

                InferType::VariableFunction(params, box ret)
            }

            InferType::Variable(type_var2) if type_var == type_var2 => replacement.clone(),

            _ => ty.clone(),
        }
    }

    crate fn compose(&self, other: Substitution) -> Substitution {
        let mut out = HashMap::new();

        for (type_var, ty) in &self.solutions {
            out.insert(*type_var, other.apply_type(ty));
        }

        out.extend(other.solutions);

        Substitution::new(out)
    }
}

impl<T: AsRef<[(usize, InferType)]>> From<T> for Substitution {
    fn from(tuples: T) -> Substitution {
        let mut map = HashMap::new();

        for (var, ty) in tuples.as_ref() {
            map.insert(TypeVar::new(*var), ty.clone());
        }

        Substitution { solutions: map }
    }
}

#[cfg(test)]
mod tests {
    use super::Substitution;
    use crate::ir::annotated::{InferType, TypeVar};
    use crate::Type;

    #[test]
    fn substitutes_type_variable() {
        let substitution = Substitution::from_pair(TypeVar::new(1), InferType::i32());
        assert_eq!(
            substitution.apply_type(&InferType::var(1)),
            InferType::i32()
        );
    }

    #[test]
    fn substitutes_type_variable_in_functions() {
        let substitution = Substitution::from([(1, InferType::i32()), (2, InferType::bool())]);

        let applied = substitution.apply_type(&InferType::variable_function(
            vec![InferType::var(1)],
            InferType::var(2),
        ));

        assert_eq!(
            applied,
            InferType::variable_function(vec![InferType::i32()], InferType::bool())
        );
    }
}
