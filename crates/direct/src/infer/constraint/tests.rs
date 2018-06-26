use super::{Constraint, Constraints};
use crate::infer::UnifyTable;
use crate::ir::annotated::{Annotated, Expression};
use crate::ir::InferType as Type;

type Term = Annotated<Expression>;

fn types() -> UnifyTable {
    UnifyTable::new()
}

#[test]
fn constrains_int() {
    let mut types = types();

    let t1 = types.fresh();
    let term = Term::integer(t1.clone(), 1);

    assert_eq!(
        term.constraints(),
        Constraints(Constraint(t1, Type::integer()))
    )
}

#[test]
fn constrains_bool() {
    let mut types = types();

    let t1 = types.fresh();
    let term = Term::bool(t1.clone(), true);

    assert_eq!(
        term.constraints(),
        Constraints(Constraint(t1, Type::bool()))
    )
}

#[test]
fn constrains_var() {
    let mut types = types();

    let t1 = types.fresh();
    let term = Term::var(t1, 0);

    assert_eq!(term.constraints(), Constraints::empty())
}

#[test]
fn constrains_app() {
    let mut types = types();

    let t1 = types.fresh();
    let t2 = types.fresh();
    let t3 = types.fresh();

    let func = Term::var(t2.clone(), 0);
    let arg = Term::var(t3.clone(), 1);
    let application = Term::apply(t1.clone(), func, vec![arg]);

    let expected = Constraints::empty()
        + Constraint(
            t2.clone(),
            Type::variable_function(vec![t3.clone()], t1.clone()),
        );

    assert_eq!(application.constraints(), expected);
}
