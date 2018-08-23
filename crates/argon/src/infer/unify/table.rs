use crate::prelude::*;

use super::Unify;
use codespan::ByteSpan;
use crate::annotated::{Annotated, RawTypeVar, TypeVar};
use crate::infer::{Constraints, Substitution};
use crate::pos::Spanned;
use crate::{CompileError, InferType};
use ena::unify::{InPlaceUnificationTable, UnifyKey, UnifyValue};
use log::*;
use std::collections::BTreeSet;

impl UnifyKey for TypeVar {
    type Value = InferType;

    fn index(&self) -> u32 {
        self.node.var as u32
    }

    fn from_index(u: u32) -> TypeVar {
        RawTypeVar { var: u as usize }.synthetic("TODO")
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
            )
                if lhs.node == rhs.node =>
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

    crate fn synthetic(&mut self) -> InferType {
        let next = self.table.len();
        trace!(target: "argon::unify", "Generating <T{}>", next);
        let ty = InferType::Variable(RawTypeVar { var: next }.synthetic("synthetic"));
        let key = self.table.new_key(ty.clone());
        self.keys.insert(key);
        ty
    }

    crate fn fresh(&mut self, span: ByteSpan) -> InferType {
        let next = self.table.len();
        trace!(target: "argon::unify", "Generating <T{}>", next);
        let ty = InferType::Variable(RawTypeVar { var: next }.with_span(span));
        let key = self.table.new_key(ty.clone());
        self.keys.insert(key);
        ty
    }

    crate fn annotate_fresh<T>(&mut self, item: T, span: ByteSpan) -> Annotated<T> {
        let ty = self.fresh(span);
        Annotated { item, ty }
    }

    crate fn unify(&self, constraints: Constraints) -> Result<Substitution, CompileError> {
        trace!(target: "argon::unify", "Unifying {:#?}", constraints);

        let UnifyTable { table, keys } = self;

        let unify = Unify {
            table: table.clone(),
            constraints,
            keys: keys.clone(),
        };

        unify.unify()
    }
}
