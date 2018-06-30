use crate::compile::math::MathOperator;
use crate::ir::{FunctionModifiers, Spanned, Type};
use crate::lexer::Tok;
use nan_preserving_float::F64;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt;

#[derive(Clone, PartialEq, new)]
pub struct RawIdentifier<'input> {
    pub name: Cow<'input, str>,
}

impl RawIdentifier<'input> {
    pub fn name(&self) -> &str {
        self.name.borrow()
    }

    fn into_owned(&self) -> RawIdentifier<'static> {
        RawIdentifier {
            name: Cow::Owned(self.name.clone().into_owned()),
        }
    }
}

pub type Identifier<'input> = Spanned<RawIdentifier<'input>>;

impl Spanned<RawIdentifier<'input>> {
    crate fn borrow(&self) -> Spanned<&str> {
        Spanned {
            node: self.node.name.borrow(),
            span: self.span,
        }
    }

    crate fn into_owned(&self) -> Spanned<RawIdentifier<'static>> {
        Spanned {
            node: self.node.into_owned(),
            span: self.span.clone(),
        }
    }
}

pub fn ident<'input>(name: Cow<'input, str>) -> RawIdentifier<'input> {
    RawIdentifier::new(name)
}

impl fmt::Debug for RawIdentifier<'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(PartialEq, Clone, new)]
pub struct Parameter<'input> {
    pub name: Identifier<'input>,
    pub ty: Spanned<Type>,
}

impl Parameter<'input> {
    fn into_owned(&self) -> Parameter<'static> {
        Parameter {
            name: self.name.into_owned(),
            ty: self.ty.clone(),
        }
    }
}

impl fmt::Debug for Parameter<'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {:?}", self.name, self.ty)
    }
}

#[derive(PartialEq, Clone)]
pub struct Parameters<'input> {
    pub list: Vec<Parameter<'input>>,
}

impl Parameters<'input> {
    crate fn iter(&self) -> impl Iterator<Item = (Spanned<&str>, &Spanned<Type>)> {
        self.list.iter().map(|arg| (arg.name.borrow(), &arg.ty))
    }

    crate fn into_owned(&self) -> Parameters<'static> {
        let list = self.list.iter().map(|p| p.into_owned()).collect();
        Parameters { list }
    }
}

impl fmt::Debug for Parameters<'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.list.iter()).finish()
    }
}

impl Parameters<'input> {
    pub fn new(list: Vec<Parameter<'input>>) -> Parameters<'input> {
        Parameters { list }
    }

    crate fn empty() -> Parameters<'input> {
        Parameters { list: vec![] }
    }

    crate fn from_parser(
        arg: Parameter<'input>,
        rest: Vec<Parameter<'input>>,
    ) -> Parameters<'input> {
        let mut list = vec![arg];
        list.extend(rest);
        Parameters { list }
    }
}

#[derive(PartialEq, Clone)]
pub struct Function<'input> {
    pub name: Identifier<'input>,
    pub args: Parameters<'input>,
    pub ret: Spanned<Type>,
    pub body: Block<'input>,
    pub modifiers: FunctionModifiers,
    crate mappings: BTreeMap<String, u32>,
}

impl Function<'input> {
    pub fn new(
        name: Identifier<'input>,
        args: Parameters<'input>,
        ret: Spanned<Type>,
        body: Block<'input>,
    ) -> Function<'input> {
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

    pub fn into_owned(&self) -> Function<'static> {
        let Function {
            name,
            args,
            ret,
            body,
            modifiers,
            mappings,
        } = self;

        Function {
            name: name.into_owned(),
            args: args.into_owned(),
            ret: ret.clone(),
            body: body.into_owned(),
            modifiers: modifiers.clone(),
            mappings: mappings.clone(),
        }
    }

    pub fn exported(mut self) -> Function<'input> {
        self.modifiers.export = true;
        self
    }
}

fn function_mappings(args: &Parameters<'input>) -> BTreeMap<String, u32> {
    let mut map = BTreeMap::new();

    for (i, (name, _)) in args.iter().enumerate() {
        map.insert(name.node.to_string(), i as u32);
    }

    map
}

impl fmt::Debug for Function<'input> {
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
pub struct Module<'input> {
    pub funcs: Vec<Function<'input>>,
}

impl Module<'input> {
    pub fn into_owned(&self) -> Module<'static> {
        Module {
            funcs: self.funcs.iter().map(|f| f.into_owned()).collect(),
        }
    }
}

#[derive(PartialEq, Clone, new)]
pub struct Block<'input> {
    pub expressions: Vec<Expression<'input>>,
}

impl Block<'input> {
    pub fn into_owned(&self) -> Block<'static> {
        let expressions = self.expressions.iter().map(|e| e.into_owned()).collect();
        Block { expressions }
    }
}

impl fmt::Debug for Block<'input> {
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
pub enum Expression<'input> {
    Const(ConstExpression),
    VariableAccess(Identifier<'input>),
    Binary(
        MathOperator,
        Spanned<Tok<'input>>,
        Box<BinaryExpression<'input>>,
    ),
}

impl Expression<'input> {
    pub fn binary(
        op: Spanned<Tok<'input>>,
        expr: Box<BinaryExpression<'input>>,
    ) -> Expression<'input> {
        let operator = match op.node {
            Tok::Add => MathOperator::Add,
            Tok::Sub => MathOperator::Sub,
            Tok::Mul => MathOperator::Mul,
            Tok::Div => MathOperator::Div,

            _ => unreachable!(),
        };

        Expression::Binary(operator, op, expr)
    }

    pub fn into_owned(&self) -> Expression<'static> {
        use self::Expression::*;

        match self {
            Const(expr) => Const(expr.clone()),
            VariableAccess(id) => VariableAccess(id.into_owned()),
            Binary(op, spanned, expr) => Binary(
                op.clone(),
                spanned.into_owned(),
                Box::new(expr.into_owned()),
            ),
        }
    }
}

impl fmt::Debug for Expression<'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value: &dyn fmt::Debug = match self {
            Expression::Const(constant) => constant,
            Expression::VariableAccess(id) => id,
            Expression::Binary(op, tok, box BinaryExpression { lhs, rhs }) => {
                return write!(f, "{:?} {:?} {:?}", lhs, op, rhs);
            }
        };

        write!(f, "{:?}", value)
    }
}

#[derive(PartialEq, Clone, new)]
pub struct BinaryExpression<'input> {
    pub lhs: Expression<'input>,
    pub rhs: Expression<'input>,
}

impl BinaryExpression<'input> {
    pub fn into_owned(&self) -> BinaryExpression<'static> {
        BinaryExpression {
            lhs: self.lhs.into_owned(),
            rhs: self.rhs.into_owned(),
        }
    }
}

impl fmt::Debug for BinaryExpression<'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} + {:?}", self.lhs, self.rhs)
    }
}
