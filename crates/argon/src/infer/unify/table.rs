use super::Unify;
use crate::annotated::{Annotated, TypeVar};
use crate::infer::{Constraints, Substitution};
use crate::pos::Spanned;
use crate::{CompileError, InferType};
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
            (
                InferType::Resolved(lhs @ Spanned { .. }),
                InferType::Resolved(rhs @ Spanned { .. }),
            ) if lhs.node == rhs.node =>
            {
                Ok(InferType::Resolved(lhs.clone()))
            }

            (InferType::Variable(..), other @ InferType::Resolved(..)) => Ok(other.clone()),
            (other @ InferType::Resolved(..), InferType::Variable(..)) => Ok(other.clone()),

            (InferType::Variable(..), other @ InferType::Variable(..)) => Ok(other.clone()),

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
