mod debug;

use parity_wasm::elements::{self, Opcode};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MathType {
    I32,
    I64,
    U32,
    U64,
    F32,
    F64,
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
