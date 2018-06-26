use crate::annotated::{self, Annotated, TypeVar};
use crate::ir::InferType;
use std::collections::BTreeMap;
use std::fmt;

#[cfg(test)]
mod test_helpers;

#[derive(Eq, PartialEq)]
crate struct Substitution {
    solutions: BTreeMap<TypeVar, InferType>,
}

impl Substitution {
    crate fn empty() -> Substitution {
        Substitution {
            solutions: BTreeMap::new(),
        }
    }

    crate fn set(&mut self, key: TypeVar, ty: InferType) {
        self.solutions.insert(key, ty);
    }

    crate fn apply_module(&self, module: annotated::Module<'input>) -> annotated::Module<'input> {
        let funcs = module
            .funcs
            .into_iter()
            .map(|f| self.apply_function(f))
            .collect();

        annotated::Module { funcs }
    }

    crate fn apply_function(
        &self,
        annotated::Function {
            name,
            params,
            symbols,
            ret,
            body,
            modifiers,
        }: annotated::Function<'input>,
    ) -> annotated::Function<'input> {
        annotated::Function {
            name,
            params,
            symbols,
            ret,
            body: self.apply_block(body),
            modifiers,
        }
    }

    crate fn apply_block(&self, block: Annotated<annotated::Block>) -> Annotated<annotated::Block> {
        let mut exprs: Vec<Annotated<annotated::Expression>> = vec![];

        for expr in block.item.expressions {
            exprs.push(self.apply_expr(expr));
        }

        Annotated {
            item: annotated::Block { expressions: exprs },
            ty: self.apply_ty(block.ty),
        }
    }

    crate fn apply_expr(
        &self,
        Annotated { item, ty }: Annotated<annotated::Expression>,
    ) -> Annotated<annotated::Expression> {
        let ty = self.apply_ty(ty);

        match item {
            c @ annotated::Expression::Const(..) => c.annotate(ty),
            v @ annotated::Expression::VariableAccess(..) => v.annotate(ty),
            annotated::Expression::Apply(box _expr, _params) => unimplemented!(),
            annotated::Expression::Binary {
                operator,
                box lhs,
                box rhs,
            } => annotated::Expression::Binary {
                operator,
                lhs: box self.apply_expr(lhs),
                rhs: box self.apply_expr(rhs),
            }.annotate(ty),
        }
    }

    crate fn apply_ty(&self, ty: InferType) -> InferType {
        match ty {
            InferType::Variable(var) => self.solutions[&var].clone(),
            r @ InferType::Resolved(..) => r,

            other => panic!(
                "Unexpected {:?}; should have been eliminated in type inference",
                other
            ),
        }
    }
}

impl std::ops::Index<TypeVar> for Substitution {
    type Output = InferType;

    fn index(&self, key: TypeVar) -> &InferType {
        self.solutions.get(&key).unwrap()
    }
}

impl std::ops::IndexMut<TypeVar> for Substitution {
    fn index_mut(&mut self, key: TypeVar) -> &mut InferType {
        self.solutions.get_mut(&key).unwrap()
    }
}

impl fmt::Debug for Substitution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self.solutions.iter()).finish()
    }
}
