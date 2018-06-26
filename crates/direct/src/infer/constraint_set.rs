use super::constraint::Constraint;
use std::collections::BTreeSet;
use std::ops::{Add, AddAssign};

#[derive(Debug, Eq, PartialEq)]
crate struct Constraints {
    crate constraints: BTreeSet<Constraint>,
}

impl IntoIterator for &'input Constraints {
    type Item = &'input Constraint;
    type IntoIter = std::collections::btree_set::Iter<'input, Constraint>;

    fn into_iter(self) -> std::collections::btree_set::Iter<'input, Constraint> {
        self.constraints.iter()
    }
}

#[allow(non_snake_case)]
crate fn Constraints(initial: Constraint) -> Constraints {
    let mut constraints = BTreeSet::new();
    constraints.insert(initial);
    Constraints { constraints }
}

impl Constraints {
    crate fn empty() -> Constraints {
        Constraints {
            constraints: BTreeSet::new(),
        }
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