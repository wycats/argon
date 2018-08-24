use super::{Constraint, Constraints};
use crate::infer::UnifyTable;
use crate::ir::annotated::{Annotated, Expression};
use crate::ir::InferType as Type;
use crate::prelude::*;

type Term = Annotated<Expression>;

fn types() -> UnifyTable {
    UnifyTable::new()
}

#[test]
fn constrains_int() {
    let mut types = types();

    let t1 = types.synthetic();
    let term = Term::integer(t1.clone(), 1);

    assert_eq!(
        term.constraints(),
        Constraints(Constraint::double(t1, Type::integer(&().synthetic("test"))))
    )
}

#[test]
fn constrains_bool() {
    let mut types = types();

    let t1 = types.synthetic();
    let term = Term::bool(t1.clone(), true);

    assert_eq!(
        term.constraints(),
        Constraints(Constraint::double(t1, Type::bool()))
    )
}

#[test]
fn constrains_var() {
    let mut types = types();

    let t1 = types.synthetic();
    let term = Term::var(t1, 0);

    assert_eq!(term.constraints(), Constraints::empty())
}

#[test]
fn constrains_app() {
    let mut types = types();

    let t1 = types.synthetic();
    let t2 = types.synthetic();
    let t3 = types.synthetic();

    let func = Term::var(t2.clone(), 0);
    let arg = Term::var(t3.clone(), 1);
    let application = Term::apply(t1.clone(), func, vec![arg]);

    let expected = Constraints::empty() + Constraint::double(
        t2.clone(),
        Type::variable_function(vec![t3.clone()], t1.clone()),
    );

    assert_eq!(application.constraints(), expected);
}
