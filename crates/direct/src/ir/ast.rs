use crate::compile::math::{f64_to_f32, MathOperator, MathType};
use crate::ir::shared;
use crate::ir::{typed, FunctionModifiers, Spanned, Type};
use nan_preserving_float::F64;
use std::collections::BTreeMap;
use std::fmt;

#[derive(PartialEq, Clone, new)]
pub struct RawIdentifier<'input> {
    pub name: &'input str,
}

pub type Identifier<'input> = Spanned<RawIdentifier<'input>>;

impl Spanned<RawIdentifier<'input>> {
    crate fn as_ref(&self) -> Spanned<&'input str> {
        Spanned {
            node: self.node.name,
            span: self.span,
        }
    }
}

pub fn ident<'input>(name: &'input str) -> RawIdentifier<'input> {
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
    pub ty: Type,
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
    crate fn iter(&self) -> impl Iterator<Item = (Spanned<&str>, &Type)> {
        self.list.iter().map(|arg| (arg.name.as_ref(), &arg.ty))
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
    pub ret: Type,
    pub body: Block<'input>,
    pub modifiers: FunctionModifiers,
    crate mappings: BTreeMap<String, u32>,
}

impl Function<'input> {
    pub fn new(
        name: Identifier<'input>,
        args: Parameters<'input>,
        ret: Type,
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

#[derive(PartialEq, Clone, new)]
pub struct Block<'input> {
    pub expressions: Vec<Expression<'input>>,
}

impl fmt::Debug for Block<'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.expressions.iter()).finish()
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum ConstExpression {
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
    crate fn to_i32(&self) -> i32 {
        match self {
            ConstExpression::Integer(int) => *int,
            ConstExpression::Float(float) if is_float_int(*float) => float.to_float() as i32,

            _ => panic!("Cannot convert {:?} to an integer"),
        }
    }

    crate fn to_u32(&self) -> u32 {
        match self {
            ConstExpression::Integer(int) if *int >= 0 => *int as u32,
            ConstExpression::Float(float) if is_float_uint(*float) => float.to_float() as u32,

            _ => panic!("Cannot convert {:?} to an integer"),
        }
    }

    crate fn into_typed_expression(self, ty: Type) -> typed::TypedExpression {
        let expr = match self {
            ConstExpression::Integer(integer) => match ty {
                Type::Math(MathType::I32) => {
                    typed::Expression::Const(shared::ConstExpression::I32(integer as i32))
                }

                Type::Math(MathType::I64) => {
                    typed::Expression::Const(shared::ConstExpression::I64(integer as i64))
                }

                Type::Math(MathType::U32) => {
                    typed::Expression::Const(shared::ConstExpression::U32(integer as u32))
                }

                Type::Math(MathType::U64) => {
                    typed::Expression::Const(shared::ConstExpression::U64(integer as u64))
                }

                other => panic!("BUG: Cannot convert an integer into {:?} (should have been eliminated by type inference)", other)
            },

            ConstExpression::Float(float) => match ty {
                Type::Math(MathType::F32) => {
                    typed::Expression::Const(shared::ConstExpression::F32(f64_to_f32(float)))
                }

                Type::Math(MathType::F64) => {
                    typed::Expression::Const(shared::ConstExpression::F64(float))
                }

                other => panic!("BUG: Cannot convert a float into {:?} (should have been eliminated by type inference)", other)
            }

            ConstExpression::Bool(bool) => panic!("unimplemented bool")
        };

        typed::TypedExpression::new(expr, ty)
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
    Binary(MathOperator, Box<BinaryExpression<'input>>),
}

impl fmt::Debug for Expression<'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value: &dyn fmt::Debug = match self {
            Expression::Const(constant) => constant,
            Expression::VariableAccess(id) => id,
            Expression::Binary(op, box BinaryExpression { lhs, rhs }) => {
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

impl fmt::Debug for BinaryExpression<'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} + {:?}", self.lhs, self.rhs)
    }
}
