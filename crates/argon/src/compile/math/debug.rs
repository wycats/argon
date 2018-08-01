use crate::prelude::*;

use super::{MathOperator, MathType};

impl fmt::Debug for MathType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathType::I32 => write!(f, "i32"),
            MathType::I64 => write!(f, "i64"),
            MathType::U32 => write!(f, "u32"),
            MathType::U64 => write!(f, "u64"),
            MathType::F32 => write!(f, "f32"),
            MathType::F64 => write!(f, "f64"),
        }
    }
}

impl fmt::Debug for MathOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let operator = match self {
            MathOperator::Add => "+",
            MathOperator::Sub => "-",
            MathOperator::Mul => "*",
            MathOperator::Div => "/",
            MathOperator::Rem => "%",
            MathOperator::And => "&",
            MathOperator::Or => "|",
            MathOperator::Xor => "^",
            MathOperator::ShiftLeft => "<<",
            MathOperator::ShiftRight => ">>",
        };

        write!(f, "{}", operator)
    }
}
