crate use super::constraint_set::Constraints;
use crate::ir::InferType;

#[cfg(test)]
mod tests;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
crate struct Constraint {
    crate left: InferType,
    crate right: InferType,
}

impl Constraint {
    crate fn new(left: InferType, right: InferType) -> Constraint {
        Constraint { left, right }
    }
}

impl std::ops::Add for Constraint {
    type Output = Constraints;

    fn add(self, rhs: Constraint) -> Constraints {
        Constraints(self) + rhs
    }
}

#[allow(non_snake_case)]
crate fn Constraint(left: InferType, right: InferType) -> Constraint {
    Constraint { left, right }
}
