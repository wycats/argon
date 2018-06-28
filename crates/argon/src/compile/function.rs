use super::body::compile_body;
use crate::{annotated, MathType, Type};
use parity_wasm::{builder, elements};

crate fn compile_function(
    function: builder::FunctionBuilder,
    input: &annotated::Function,
) -> builder::FunctionDefinition {
    let mut signature = function.signature();

    for ty in input.params.iter() {
        signature = signature.with_param(parameter_type(&ty.node))
    }

    signature = signature.with_return_type(wasm_type(&input.ret.node));

    let instructions = compile_body(&input.body, input);

    signature
        .build()
        .body()
        .with_opcodes(elements::Opcodes::new(instructions))
        .build()
        .build()
}

fn parameter_type(input: &Type) -> elements::ValueType {
    wasm_type(input).expect("void is not allowed as a parameter type")
}

fn wasm_type(input: &Type) -> Option<elements::ValueType> {
    match input {
        Type::Math(ty) => match ty {
            MathType::F32 => Some(elements::ValueType::F32),
            MathType::F64 => Some(elements::ValueType::F64),
            MathType::U32 | MathType::I32 => Some(elements::ValueType::I32),
            MathType::U64 | MathType::I64 => Some(elements::ValueType::I64),
        },
        Type::Bool => Some(elements::ValueType::I32),
        Type::Function(..) | Type::Apply(..) => {
            panic!("Cannot convert a function into a wasm type")
        }
        Type::Void => None,
    }
}
