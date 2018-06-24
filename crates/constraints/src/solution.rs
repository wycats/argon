use crate::collections::{TypeMap, TypeSet};
use crate::{Type, TypeError, TypeVar};
use std::ops::{Add, AddAssign};

pub enum TypeStatus {
    Union(TypeSet),
    Unbounded,
}

impl TypeStatus {
    pub fn contains(&self, ty: &Type) -> bool {
        match self {
            TypeStatus::Unbounded => true,
            TypeStatus::Union(set) => set.has(ty),
        }
    }
}

pub struct Solution {
    unsolved: TypeMap<TypeStatus>,
    solved: TypeMap<Type>,
}

impl Solution {
    pub fn empty() -> Solution {
        Solution {
            solved: TypeMap::empty(),
            unsolved: TypeMap::empty(),
        }
    }

    pub fn for_vars(vars: impl IntoIterator<Item = TypeVar>) -> Solution {
        let mut unsolved = TypeMap::empty();

        for var in vars {
            unsolved[var] = TypeStatus::Unbounded;
        }

        Solution {
            unsolved,
            solved: TypeMap::empty(),
        }
    }

    pub fn solve(&mut self, var: TypeVar, ty: Type) -> Result<(), TypeError> {
        debug_assert!(self.solved.has(var) && !self.unsolved.has(var));

        let status = self.unsolved.take(var);

        if status.contains(&ty) {
            self.solved[var] = ty;
            Ok(())
        } else {
            Err(TypeError::Mismatch)
        }
    }
}

impl Add<(TypeVar, Type)> for Solution {
    type Output = Solution;

    fn add(mut self, other: (TypeVar, Type)) -> Solution {
        self.solved[other.0] = other.1;
        self
    }
}

impl AddAssign<(TypeVar, Type)> for Solution {
    fn add_assign(&mut self, other: (TypeVar, Type)) {
        self.solved[other.0] = other.1;
    }
}
