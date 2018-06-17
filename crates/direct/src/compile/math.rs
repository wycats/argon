use crate::ir::Type;
use parity_wasm::elements;

#[derive(Copy, Clone, Eq, PartialEq)]
crate enum MathType {
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

crate fn plus_op(ty: MathType) -> elements::Opcode {
    match ty {
        MathType::I32 => elements::Opcode::I32Add,
        MathType::I64 => elements::Opcode::I64Add,
        MathType::F32 => elements::Opcode::F32Add,
        MathType::F64 => elements::Opcode::F64Add,
    }
}
