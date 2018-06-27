crate mod add;

use super::constraint::Constraint;
use std::collections::BTreeSet;

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
