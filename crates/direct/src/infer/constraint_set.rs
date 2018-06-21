use super::constraint::Constraint;
use crate::ir::annotated::{self, Annotated};
use crate::ir::InferType;
use std::collections::HashSet;
use std::ops::{Add, AddAssign};

#[derive(Debug, Eq, PartialEq)]
crate struct Constraints {
    crate constraints: HashSet<Constraint>,
}

crate fn Constraints(initial: Constraint) -> Constraints {
    let mut constraints = HashSet::new();
    constraints.insert(initial);
    Constraints { constraints }
}

impl Constraints {
    crate fn empty() -> Constraints {
        Constraints {
            constraints: HashSet::new(),
        }
    }

    crate fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }

    crate fn take_head(self) -> (Constraint, Constraints) {
        let mut constraints = self.constraints.into_iter();
        let head = constraints.next().unwrap();
        let tail = constraints.collect();

        (head, Constraints { constraints: tail })
    }
}

impl Add for Constraints {
    type Output = Constraints;

    fn add(self, rhs: Constraints) -> Constraints {
        let mut constraints = self.constraints;

        for constraint in rhs.constraints.into_iter() {
            constraints.insert(constraint);
        }

        Constraints { constraints }
    }
}

impl Add<Constraint> for Constraints {
    type Output = Constraints;

    fn add(self, rhs: Constraint) -> Constraints {
        let mut constraints = self.constraints;
        constraints.insert(rhs);

        Constraints { constraints }
    }
}

impl AddAssign for Constraints {
    fn add_assign(&mut self, rhs: Constraints) {
        for constraint in rhs.constraints.into_iter() {
            self.constraints.insert(constraint);
        }
    }
}

impl AddAssign<Constraint> for Constraints {
    fn add_assign(&mut self, rhs: Constraint) {
        self.constraints.insert(rhs);
    }
}
