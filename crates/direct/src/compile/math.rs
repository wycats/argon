use crate::ir::Type;
use parity_wasm::elements::{self, Opcode};
use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum MathType {
    I32,
    I64,
    U32,
    U64,
    F32,
    F64,
}

impl fmt::Debug for MathType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MathType::I32 => "i32",
                MathType::I64 => "i64",
                MathType::U32 => "u32",
                MathType::U64 => "u64",
                MathType::F32 => "f32",
                MathType::F64 => "f64",
            }
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
crate enum BinaryType {
    CoerceLeft(MathType),
    CoerceRight(MathType),
    Same(MathType),
    Incompatible(Type, Type),
}

crate fn binary_op_type(lhs: Type, rhs: Type) -> BinaryType {
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
