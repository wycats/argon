use crate::{ConstExpression, FunctionModifiers, MathOperator, Type};
use std::collections::HashSet;

#[derive(Debug)]
pub struct PossibleTypes {
    types: HashSet<Type>,
}

#[derive(Debug)]
pub enum Expression {
    Const(ConstExpression),

    VariableAccess(u32),
    Binary(MathOperator, Box<BinaryExpression>),
}

#[derive(Debug)]
pub struct TypedModule<'input> {
    pub funcs: Vec<TypedFunction<'input>>,
}

#[derive(Debug)]
pub struct TypedFunction<'input> {
    pub name: &'input str,
    pub params: Vec<Type>,
    pub symbols: Vec<&'input str>,
    pub ret: Type,
    pub body: TypedBlock,
    pub modifiers: FunctionModifiers,
}

#[derive(Debug)]
pub struct TypedBlock {
    expressions: Vec<TypedExpression>,
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub lhs: TypedExpression,
    pub rhs: TypedExpression,
}

#[allow(unused)]
#[derive(Debug, new)]
pub struct TypedExpression {
    pub expression: Expression,
    pub ty: PossibleTypes,
}
