use super::expression::compile_expression;
use crate::annotated;
use parity_wasm::elements;

crate fn compile_body(
    input: &annotated::Block,
    function: &annotated::Function,
) -> Vec<elements::Opcode> {
    let mut instructions = vec![];

    for expression in &input.expressions {
        compile_expression(&mut instructions, expression, function);
    }

    instructions.push(elements::Opcode::End);

    instructions
}
