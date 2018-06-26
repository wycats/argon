use super::{Constraint, Constraints};
use std::ops::{Add, AddAssign};

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
