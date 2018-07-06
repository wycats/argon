use crate::prelude::*;

use crate::compile::math::MathOperator;
use crate::ir::{FunctionModifiers, Spanned, Type};
use crate::lexer::{IdentifierId, Tok, Token};

pub type Identifier = Token;

#[derive(PartialEq, Clone, new)]
pub struct Parameter {
    pub name: Identifier,
    pub ty: Spanned<Type>,
}

impl fmt::Debug for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {:?}", self.name, self.ty)
    }
}

#[derive(PartialEq, Clone)]
pub struct Parameters {
    pub list: Vec<Parameter>,
}

impl Parameters {
    crate fn iter(&self) -> impl Iterator<Item = (Token, &Spanned<Type>)> {
        self.list.iter().map(|arg| (arg.name, &arg.ty))
    }
}

impl fmt::Debug for Parameters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.list.iter()).finish()
    }
}

impl Parameters {
    pub fn new(list: Vec<Parameter>) -> Parameters {
        Parameters { list }
    }

    crate fn empty() -> Parameters {
        Parameters { list: vec![] }
    }

    crate fn from_parser(arg: Parameter, rest: Vec<Parameter>) -> Parameters {
        let mut list = vec![arg];
        list.extend(rest);
        Parameters { list }
    }
}

#[derive(PartialEq, Clone)]
pub struct Function {
    pub name: Identifier,
    pub args: Parameters,
    pub ret: Spanned<Type>,
    pub body: Block,
    pub modifiers: FunctionModifiers,
    crate mappings: BTreeMap<IdentifierId, u32>,
}

impl Function {
    pub fn new(name: Identifier, args: Parameters, ret: Spanned<Type>, body: Block) -> Function {
        let mappings = function_mappings(&args);

        Function {
            name,
            args,
            ret,
            body,
            modifiers: FunctionModifiers::new(),
            mappings,
        }
    }

    pub fn exported(mut self) -> Function {
        self.modifiers.export = true;
        self
    }
}

fn function_mappings(args: &Parameters) -> BTreeMap<IdentifierId, u32> {
    let mut map = BTreeMap::new();

    for (i, (name, _)) in args.iter().enumerate() {
        map.insert(name.to_ident(), i as u32);
    }

    map
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map()
            .entry(&"name", &self.name)
            .entry(&"args", &self.args)
            .entry(&"return", &self.ret)
            .entry(&"body", &self.body)
            .entry(&"export", &self.modifiers.export)
            .finish()
    }
}

#[derive(PartialEq, Clone, Debug, new)]
pub struct Module {
    pub funcs: Vec<Function>,
}

#[derive(PartialEq, Clone, new)]
pub struct Block {
    pub expressions: Vec<Expression>,
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.expressions.iter()).finish()
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum ConstExpression {
    Integer(Spanned<i32>),
    Float(Spanned<F64>),
    Bool(Spanned<bool>),
}

#[derive(PartialEq, Copy, Clone)]
crate enum ConstValue {
    Integer(i32),
    Float(F64),
    Bool(bool),
}

fn is_float_int(float: F64) -> bool {
    float.to_float().floor() == float.to_float()
}

fn is_float_uint(float: F64) -> bool {
    float.to_float().floor() == float.to_float() && float.to_float() >= 0.0
}

impl ConstExpression {
    crate fn value(&self) -> ConstValue {
        match self {
            ConstExpression::Integer(Spanned { node: int, .. }) => ConstValue::Integer(*int),
            ConstExpression::Float(Spanned { node: float, .. }) => ConstValue::Float(*float),
            ConstExpression::Bool(Spanned { node: boolean, .. }) => ConstValue::Bool(*boolean),
        }
    }

    crate fn to_i32(&self) -> i32 {
        match self.value() {
            ConstValue::Integer(int) => int,
            ConstValue::Float(float) if is_float_int(float) => float.to_float() as i32,

            _ => panic!("Cannot convert {:?} to an integer"),
        }
    }

    crate fn to_u32(&self) -> u32 {
        match self.value() {
            ConstValue::Integer(int) if int >= 0 => int as u32,
            ConstValue::Float(float) if is_float_uint(float) => float.to_float() as u32,

            _ => panic!("Cannot convert {:?} to an unsigned integer"),
        }
    }

    crate fn to_f32(&self) -> f32 {
        match self.value() {
            ConstValue::Integer(int) if int >= 0 => int as f32,
            ConstValue::Float(float) => float.to_float() as f32,

            _ => panic!("Cannot convert {:?} to a float"),
        }
    }

    crate fn to_f64(&self) -> f64 {
        match self.value() {
            ConstValue::Integer(int) if int >= 0 => int as f64,
            ConstValue::Float(float) => float.to_float(),

            _ => panic!("Cannot convert {:?} to a float"),
        }
    }
}

impl fmt::Debug for ConstExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConstExpression::Integer(int) => write!(f, "{:?}", *int),
            ConstExpression::Float(float) => write!(f, "{:?}", *float),
            ConstExpression::Bool(boolean) => write!(f, "{:?}", *boolean),
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum Expression {
    Const(ConstExpression),
    VariableAccess(Identifier),
    Binary(MathOperator, Token, Box<BinaryExpression>),
}

impl Expression {
    pub fn binary(op: Token, expr: Box<BinaryExpression>) -> Expression {
        let operator = match op.node {
            Tok::Add => MathOperator::Add,
            Tok::Sub => MathOperator::Sub,
            Tok::Mul => MathOperator::Mul,
            Tok::Div => MathOperator::Div,

            _ => unreachable!(),
        };

        Expression::Binary(operator, op, expr)
    }
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value: &dyn fmt::Debug = match self {
            Expression::Const(constant) => constant,
            Expression::VariableAccess(id) => id,
            Expression::Binary(op, _tok, box BinaryExpression { lhs, rhs }) => {
                return write!(f, "{:?} {:?} {:?}", lhs, op, rhs);
            }
        };

        write!(f, "{:?}", value)
    }
}

#[derive(PartialEq, Clone, new)]
pub struct BinaryExpression {
    pub lhs: Expression,
    pub rhs: Expression,
}

impl fmt::Debug for BinaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} + {:?}", self.lhs, self.rhs)
    }
}
