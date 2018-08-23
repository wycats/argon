use super::Substitution;
use crate::ir::{InferType, RawTypeVar};
use crate::prelude::*;

impl Substitution {
    crate fn from(tuples: impl AsRef<[(usize, InferType)]>) -> Substitution {
        let mut sub = Substitution::empty();

        for (var, ty) in tuples.as_ref() {
            sub.set(RawTypeVar { var: *var }.synthetic("synthetic"), ty.clone());
        }

        sub
    }
}
