use std::fmt;

#[derive(Debug)]
pub enum TypeError {
    MismatchedPlus(Type, Type),
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum ConstExpression {
    I32(i32),
    I64(i64),
    F32(u32),
    F64(u64),
}

impl fmt::Debug for ConstExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConstExpression::I32(int) => write!(f, "{:?}", *int),
            ConstExpression::I64(int) => write!(f, "{:?}", *int),
            ConstExpression::F32(float) => write!(f, "{:?}", *float as f32),
            ConstExpression::F64(float) => write!(f, "{:?}", *float as f64),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Type {
    I32,
    I64,
    F32,
    F64,
    Void,
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Type::I32 => "i32",
                Type::I64 => "i64",
                Type::F32 => "f32",
                Type::F64 => "f64",
                Type::Void => "void",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, new)]
pub struct FunctionModifiers {
    #[new(default)]
    pub export: bool,
}
