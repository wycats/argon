use crate::{types, Constraint, ExternType, Type};

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
enum TestTypes {
    Integer,
    Boolean,
}

const Integer: ExternType = ExternType(0);
const Boolean: ExternType = ExternType(1);

#[test]
fn unifies_literal() {
    pretty_env_logger::try_init();

    let mut types = types();

    // def add(x: i64) -> i64 { x + 50 }

    let constraints = Constraint::Equals(Type::var(0), Type::var(1))
        + Constraint::Equals(Type::var(0), Type::ty(Integer))
        + Constraint::Equals(Type::var(1), Type::ty(Boolean));

    // let constraints = Constraint(InferType::var(0), InferType::var(1))
    //     + Constraint(InferType::var(0), InferType::i64())
    //     + Constraint(InferType::var(1), InferType::integer());

    // let substitution = unify(constraints);
    // let expected = Substitution::from(&[(1, InferType::i64()), (0, InferType::var(1))]);

    // assert_eq!(substitution, Ok(expected));
}
