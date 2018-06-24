use super::constraint::{Constraint, Constraints};
use crate::annotated::{self, TypeVar};
use crate::ir::{typed, InferType};
use crate::shared;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

type SubstitutionMap = BTreeMap<TypeVar, InferType>;

#[derive(Eq, PartialEq)]
crate struct Substitution {
    solutions: BTreeMap<TypeVar, InferType>,
}

impl Substitution {
    crate fn new(solutions: BTreeMap<TypeVar, InferType>) -> Substitution {
        Substitution { solutions }
    }

    crate fn from(tuples: impl AsRef<[(usize, InferType)]>) -> Substitution {
        let mut map = BTreeMap::new();

        for (var, ty) in tuples.as_ref() {
            map.insert(TypeVar::new(*var), ty.clone());
        }

        Substitution { solutions: map }
    }

    crate fn empty() -> Substitution {
        Substitution {
            solutions: BTreeMap::new(),
        }
    }

    crate fn from_pair(type_var: TypeVar, ty: InferType) -> Substitution {
        let mut solutions = BTreeMap::new();
        solutions.insert(type_var, ty);

        Substitution { solutions }
    }

    crate fn set(&mut self, key: TypeVar, ty: InferType) {
        self.solutions.insert(key, ty);
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
