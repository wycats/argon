use crate::ir::Type;
use crate::InferType;
use nan_preserving_float::{F32, F64};
use parity_wasm::elements::{self, Opcode};
use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MathType {
    I32,
    I64,
    U32,
    U64,
    F32,
    F64,
}

impl MathType {
    crate fn is_integer(&self) -> bool {
        match self {
            MathType::I32 | MathType::I64 | MathType::U32 | MathType::U64 => true,
            _ => false,
        }
    }

    crate fn is_float(&self) -> bool {
        match self {
            MathType::F32 | MathType::F64 => true,
            _ => false,
        }
    }
}

impl fmt::Debug for MathType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

#[derive(Debug)]
crate enum BinaryType {
    CoerceLeft(MathType),
    CoerceRight(MathType),
    Same(MathType),
    Incompatible(Type, Type),
}

crate fn binary_op_type(lhs: InferType, rhs: InferType) -> BinaryType {
    let lhs = lhs.into_type();
    let rhs = rhs.into_type();

    match (lhs, rhs) {
        (Type::Math(lhs), Type::Math(rhs)) if lhs == rhs => BinaryType::Same(lhs),
        (left, right) => BinaryType::Incompatible(left, right),
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MathOperator {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    And,
    Or,
    Xor,
    ShiftLeft,
    ShiftRight,
}

impl fmt::Debug for MathOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

crate fn math_op(operator: MathOperator, ty: MathType) -> elements::Opcode {
    match operator {
        MathOperator::Add => match ty {
            MathType::I32 | MathType::U32 => Opcode::I32Add,
            MathType::I64 | MathType::U64 => Opcode::I64Add,
            MathType::F32 => Opcode::F32Add,
            MathType::F64 => Opcode::F64Add,
        },

        MathOperator::Sub => match ty {
            MathType::I32 | MathType::U32 => Opcode::I32Sub,
            MathType::I64 | MathType::U64 => Opcode::I64Sub,
            MathType::F32 => Opcode::F32Sub,
            MathType::F64 => Opcode::F64Sub,
        },

        MathOperator::Mul => match ty {
            MathType::I32 | MathType::U32 => Opcode::I32Mul,
            MathType::I64 | MathType::U64 => Opcode::I64Mul,
            MathType::F32 => Opcode::F32Mul,
            MathType::F64 => Opcode::F64Mul,
        },

        MathOperator::Div => match ty {
            MathType::I32 => Opcode::I32DivS,
            MathType::I64 => Opcode::I64DivS,
            MathType::U32 => Opcode::I32DivU,
            MathType::U64 => Opcode::I64DivU,
            MathType::F32 => Opcode::F32Div,
            MathType::F64 => Opcode::F64Div,
        },

        _ => unimplemented!("Unimplemented operator {:?} for {:?}", operator, ty),
    }
}

crate fn f64_to_f32(input: F64) -> F32 {
    F32::from_float(input.to_float() as f32)
}
