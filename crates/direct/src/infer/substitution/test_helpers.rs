use super::Substitution;
use crate::ir::{InferType, TypeVar};
use std::collections::BTreeMap;

impl Substitution {
    crate fn from(tuples: impl AsRef<[(usize, InferType)]>) -> Substitution {
        let mut map = BTreeMap::new();

        for (var, ty) in tuples.as_ref() {
            map.insert(TypeVar::new(*var), ty.clone());
        }

        Substitution { solutions: map }
    }
}
