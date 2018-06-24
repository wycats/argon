use crate::collections::{TypeSet, TypeVarSet};
use crate::Type;
use std::collections::BTreeSet;
use std::ops::{Add, AddAssign};

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
pub enum Constraint {
    Equals(Type, Type),
    Subset(Type, TypeSet),
}

impl Constraint {
    pub fn vars(&self) -> TypeVarSet {
        match self {
            Constraint::Equals(left, right) => left.vars() + right.vars(),
            Constraint::Subset(ty, types) => ty.vars() + types.vars(),
        }
    }
}

pub struct Constraints {
    set: BTreeSet<Constraint>,
}

impl Constraints {
    pub fn empty() -> Constraints {
        Constraints {
            set: BTreeSet::new(),
        }
    }

    pub fn vars(&self) -> TypeVarSet {
        let mut vars = TypeVarSet::empty();

        for item in &self.set {
            vars += item.vars()
        }

        vars
    }
}

impl Add for Constraint {
    type Output = Constraints;

    fn add(self, other: Constraint) -> Constraints {
        let mut set = BTreeSet::new();
        set.insert(self);
        set.insert(other);
        Constraints { set }
    }
}

impl Add<Constraint> for Constraints {
    type Output = Constraints;

    fn add(mut self, other: Constraint) -> Constraints {
        self.set.insert(other);
        self
    }
}

impl AddAssign<Constraint> for Constraints {
    fn add_assign(&mut self, other: Constraint) {
        self.set.insert(other);
    }
}
