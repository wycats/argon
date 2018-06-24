use crate::{Constraints, Solution, TypeVar};

pub struct Unify {
    constraints: Constraints,
    solution: Solution,
}

// impl Unify {
//     pub fn new(constraints: Constraints) -> Unify {
//         let vars = constraints.vars();
//         let unsolved = TypeMap::new();

//         for var in vars {
//             unsolved[var] = TypeStatus::Unbounded;
//         }

//         unimplemented!()
//     }
// }
