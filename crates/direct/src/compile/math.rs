use crate::ir::Type;
use parity_wasm::elements::{self, Opcode};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MathType {
    I32,
    I64,
    F32,
    F64,
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
        (Type::I32, Type::I32) => BinaryType::Same(MathType::I32),
        (Type::I64, Type::I64) => BinaryType::Same(MathType::I64),
        (Type::F32, Type::F32) => BinaryType::Same(MathType::F32),
        (Type::F64, Type::F64) => BinaryType::Same(MathType::F64),
        (left, right) => BinaryType::Incompatible(left, right),
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
            MathType::I32 => Opcode::I32Add,
            MathType::I64 => Opcode::I64Add,
            MathType::F32 => Opcode::F32Add,
            MathType::F64 => Opcode::F64Add,
        },

        MathOperator::Sub => match ty {
            MathType::I32 => Opcode::I32Sub,
            MathType::I64 => Opcode::I64Sub,
            MathType::F32 => Opcode::F32Sub,
            MathType::F64 => Opcode::F64Sub,
        },

        _ => unimplemented!("Unimplemented operator {:?} for {:?}", operator, ty),
    }
}
