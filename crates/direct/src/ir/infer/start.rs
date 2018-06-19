use crate::ir::resolved;
use crate::{ast, FunctionModifiers, MathOperator, Spanned, Type};
use std::cmp::max;
use std::collections::HashSet;
use std::iter::FromIterator;

pub trait IsResolved {
    fn is_resolved(&self) -> bool;
}

pub enum Resolution {
    AlreadyResolved,
    Progress,
    NoProgress,
    Failure,
}

pub trait ResolveProgress: Sized {
    fn resolve(self, ty: Type) -> Self {
        let mut items = HashSet::new();
        items.insert(ty);
        self.make_progress(items)
    }

    fn make_progress(self, set: HashSet<Type>) -> Self;
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PossibleTypes {
    Any,
    Some(HashSet<Type>),
}

pub enum Relationship {
    Subset,
    Superset,
    Disjoint,
    Same,
}

impl PossibleTypes {
    pub fn resolved(ty: Type) -> PossibleTypes {
        let mut types = HashSet::new();
        types.insert(ty);
        PossibleTypes::Some(types)
    }

    pub fn is_resolved(&self) -> bool {
        match self {
            PossibleTypes::Any => false,
            PossibleTypes::Some(set) => set.len() == 1,
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            PossibleTypes::Any => false,
            PossibleTypes::Some(set) => set.len() == 0,
        }
    }

    pub fn relationship(&self, other: &PossibleTypes) -> Relationship {
        match (self, other) {
            (PossibleTypes::Some(left), PossibleTypes::Some(right)) => {
                if left.is_disjoint(right) {
                    Relationship::Disjoint
                } else if left == right {
                    Relationship::Same
                } else if left.is_subset(right) {
                    Relationship::Subset
                } else if left.is_superset(right) {
                    Relationship::Superset
                } else {
                    unreachable!()
                }
            }

            (PossibleTypes::Any, PossibleTypes::Any) => Relationship::Same,
            (PossibleTypes::Any, _) => Relationship::Superset,
            (PossibleTypes::Some(_), PossibleTypes::Any) => Relationship::Subset,
        }
    }

    pub fn is_subset_of(&self, other: &PossibleTypes) -> bool {
        match (self, other) {
            (PossibleTypes::Some(left), PossibleTypes::Some(right)) => left.is_subset(right),

            (PossibleTypes::Any, PossibleTypes::Any) => true,
            (PossibleTypes::Some(_), PossibleTypes::Any) => true,
            (PossibleTypes::Any, PossibleTypes::Some(_)) => false,
        }
    }

    pub fn intersect(self, other: PossibleTypes) -> (PossibleTypes, bool) {
        match (self, other) {
            (PossibleTypes::Any, PossibleTypes::Any) => (PossibleTypes::Any, false),
            (PossibleTypes::Any, other) => (other, true),

            (PossibleTypes::Some(left), PossibleTypes::Some(right)) => {
                if left == right {
                    (PossibleTypes::Some(left), false)
                } else {
                    let start = max(left.len(), right.len());

                    let mut remains = left.intersection(&right);
                    let finish = remains.clone().count();

                    (
                        PossibleTypes::Some(remains.cloned().collect()),
                        start != finish,
                    )
                }
            }
            (PossibleTypes::Some(left), PossibleTypes::Any) => (PossibleTypes::Some(left), true),
        }
    }
}

#[derive(Debug)]
pub struct Module<'input> {
    pub funcs: Vec<Function<'input>>,
}

impl IsResolved for Module<'input> {
    fn is_resolved(&self) -> bool {
        self.funcs.iter().all(|f| f.is_resolved())
    }
}

impl resolved::Module<'input> {
    fn start_inference(self) -> Module<'input> {
        Module {
            funcs: self
                .funcs
                .into_iter()
                .map(|f| f.start_inference())
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct Function<'input> {
    pub name: Spanned<&'input str>,
    pub params: Vec<Type>,
    pub symbols: Vec<Spanned<&'input str>>,
    pub ret: Type,
    pub body: Block,
    pub modifiers: FunctionModifiers,
}

impl IsResolved for Function<'input> {
    fn is_resolved(&self) -> bool {
        self.body.is_resolved()
    }
}

impl resolved::Function<'input> {
    fn start_inference(self) -> Function<'input> {
        let body = self.body.start_inference();

        Function {
            name: self.name,
            params: self.params,
            symbols: self.symbols,
            ret: self.ret,
            body,
            modifiers: self.modifiers,
        }
    }
}

#[derive(Debug)]
pub struct Block {
    pub expressions: Vec<Expression>,
}

impl IsResolved for Block {
    fn is_resolved(&self) -> bool {
        self.expressions.iter().all(|e| e.is_resolved())
    }
}

impl resolved::Block {
    fn start_inference(self) -> Block {
        Block {
            expressions: self
                .expressions
                .into_iter()
                .map(|e| e.start_inference())
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub lhs: Expression,
    pub rhs: Expression,
}

impl IsResolved for BinaryExpression {
    fn is_resolved(&self) -> bool {
        self.lhs.is_resolved() && self.rhs.is_resolved()
    }
}

impl resolved::BinaryExpression {
    fn start_inference(self) -> BinaryExpression {
        BinaryExpression {
            lhs: self.lhs.start_inference(),
            rhs: self.rhs.start_inference(),
        }
    }
}

#[derive(Debug)]
pub struct TypedExpression {
    pub expression: resolved::Expression,
    pub ty: PossibleTypes,
}

#[derive(Debug, Clone)]
pub enum ExpressionEnum {
    Const(ast::ConstExpression),
    VariableAccess(u32),
    Binary(MathOperator, Box<BinaryExpression>),
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub types: PossibleTypes,
    pub expression: ExpressionEnum,
}

impl IsResolved for Expression {
    fn is_resolved(&self) -> bool {
        match &self.types {
            PossibleTypes::Some(set) => set.len() == 1,
            _ => false,
        }
    }
}

impl resolved::Expression {
    fn start_inference(self) -> Expression {
        match self {
            resolved::Expression::VariableAccess(id) => Expression {
                expression: ExpressionEnum::VariableAccess(id),
                types: PossibleTypes::Any,
            },

            resolved::Expression::Const(val) => Expression {
                expression: ExpressionEnum::Const(val),
                types: PossibleTypes::Any,
            },

            resolved::Expression::Binary(operator, box binary) => {
                let resolved::BinaryExpression { lhs, rhs } = binary;

                let lhs = lhs.start_inference();
                let rhs = rhs.start_inference();

                Expression {
                    expression: ExpressionEnum::Binary(
                        operator,
                        Box::new(BinaryExpression { lhs, rhs }),
                    ),
                    types: PossibleTypes::Any,
                }
            }
        }
    }
}

impl ResolveProgress for Expression {
    fn make_progress(self, items: HashSet<Type>) -> Expression {
        let Expression { types, expression } = self;
        let types = PossibleTypes::Some(items);

        Expression {
            expression,
            types: types,
        }
    }
}
