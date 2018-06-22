use crate::compile::math::{binary_op_type, math_op, BinaryType};
use crate::infer::constraint::CollectConstraints;
use crate::ir::annotated::TypeVars;
use crate::ir::{annotated, typed, CompileError, ConstExpression, Type};
use crate::MathType;
use crate::{ast, resolved};
use parity_wasm::{builder, elements};

struct CodeLocation {
    /// Location (index in 'functions' section) of the signature
    signature: u32,
    /// Location (index in the 'code' section) of the body
    _body: u32,
}

pub fn compile_module(input: &ast::Module) -> Result<elements::Module, CompileError> {
    let mut vars = TypeVars::new();

    let module = resolved::resolve_module_names(input)?;
    let module = annotated::Module::from(module, &mut vars);
    trace!(target: "wasm::compile::module", "Module: {:#?}", module);
    let constraints = module.constraints();
    trace!(target: "wasm::compile::constraints", "Constraints: {:#?}", constraints);
    let substitutions = constraints.unify()?;
    trace!(target: "wasm::compile::substitutions", "Substitutions: {:#?}", substitutions);
    let module = substitutions.apply_module(module);
    trace!(target: "wasm::compile::applies", "After Substitutions: {:#?}", module);

    unimplemented!();

    /*
    let typed = module.ast_to_hir()?;

    let mut builder = builder::module();

    for func in &typed.funcs {
        let function = builder::function();
        let function = compile_function(function, func);
        let location: CodeLocation =
            unsafe { std::mem::transmute(builder.push_function(function)) };

        if func.modifiers.export {
            builder = builder
                .export()
                .field(func.name.node)
                .internal()
                .func(location.signature)
                .build();
        }
    }

    Ok(builder.build())
    */
}

fn compile_function(
    function: builder::FunctionBuilder,
    input: &typed::TypedFunction,
) -> builder::FunctionDefinition {
    let mut signature = function.signature();

    for ty in input.params.iter() {
        signature = signature.with_param(parameter_type(&ty))
    }

    signature = signature.with_return_type(wasm_type(&input.ret));

    let instructions = compile_body(&input.body, input);

    signature
        .build()
        .body()
        .with_opcodes(elements::Opcodes::new(instructions))
        .build()
        .build()
}

fn compile_body(
    input: &typed::TypedBlock,
    function: &typed::TypedFunction,
) -> Vec<elements::Opcode> {
    let mut instructions = vec![];

    for expression in input.iter() {
        compile_expression(&mut instructions, expression, function);
    }

    instructions.push(elements::Opcode::End);

    instructions
}

fn compile_expression(
    body: &mut Vec<elements::Opcode>,
    input: &typed::TypedExpression,
    function: &typed::TypedFunction,
) {
    match input {
        typed::TypedExpression { ty: _, expression } => match expression {
            typed::Expression::Const(constant) => match constant {
                ConstExpression::I32(int) => body.push(elements::Opcode::I32Const(*int)),
                ConstExpression::I64(int) => body.push(elements::Opcode::I64Const(*int)),
                ConstExpression::U32(int) => body.push(elements::Opcode::I32Const(unsafe {
                    std::mem::transmute(*int)
                })),
                ConstExpression::U64(int) => body.push(elements::Opcode::I64Const(unsafe {
                    std::mem::transmute(*int)
                })),
                ConstExpression::F32(float) => body.push(elements::Opcode::F32Const(unsafe {
                    std::mem::transmute(*float)
                })),
                ConstExpression::F64(float) => body.push(elements::Opcode::F64Const(unsafe {
                    std::mem::transmute(*float)
                })),
            },

            typed::Expression::VariableAccess(local) => {
                body.push(elements::Opcode::GetLocal(*local));
            }

            typed::Expression::Binary(operator, box typed::BinaryExpression { lhs, rhs }) => {
                let ty = binary_op_type(lhs.ty.clone(), rhs.ty.clone());

                match ty {
                    BinaryType::Same(ty) => {
                        compile_expression(body, lhs, function);
                        compile_expression(body, rhs, function);
                        body.push(math_op(*operator, ty));
                    }

                    BinaryType::CoerceLeft(_) | BinaryType::CoerceRight(_) => {
                        unimplemented!("[TODO?] No support for coercions yet")
                    }

                    BinaryType::Incompatible(lhs, rhs) => {
                        panic!("TypeError: {:?} + {:?} is invalid", lhs, rhs)
                    }
                }
            }
        },
    }
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
