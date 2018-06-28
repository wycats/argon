use crate::annotated::Annotated;
use crate::ir::Spanned;
use crate::math::math_op;
use crate::{annotated, ast, InferType, MathType, Type};
use parity_wasm::elements;
use std::mem::transmute;

crate fn compile_expression(
    body: &mut Vec<elements::Opcode>,
    Annotated { item, ty }: &Annotated<annotated::Expression>,
    function: &annotated::Function,
) {
    match item {
        annotated::Expression::Const(constant) => body.push(compile_const(constant, ty)),

        annotated::Expression::VariableAccess(local) => {
            body.push(elements::Opcode::GetLocal(*local));
        }

        annotated::Expression::Apply(..) => unimplemented!(),

        annotated::Expression::Binary {
            operator,
            box lhs,
            box rhs,
        } => {
            debug_assert!(
                lhs.ty == rhs.ty,
                "inference bug: the left and right side of a binary operator must be the same type"
            );

            compile_expression(body, lhs, function);
            compile_expression(body, rhs, function);
            body.push(math_op(*operator, lhs.ty.as_math()));
        }
    }
}

fn compile_const(constant: &ast::ConstExpression, ty: &InferType) -> elements::Opcode {
    let ty = ty.clone().into_type();

    match ty {
        Spanned {
            node: Type::Math(math),
            ..
        } => match math {
            MathType::I32 => elements::Opcode::I32Const(constant.to_i32()),
            MathType::I64 => elements::Opcode::I64Const(constant.to_i32() as i64),
            MathType::U32 => elements::Opcode::I32Const(unsafe { transmute(constant.to_u32()) }),
            MathType::U64 => {
                elements::Opcode::I64Const(unsafe { transmute(constant.to_i32() as i64) })
            }

            MathType::F32 => elements::Opcode::F32Const(unsafe { transmute(constant.to_f32()) }),
            MathType::F64 => elements::Opcode::F64Const(unsafe { transmute(constant.to_f64()) }),
        },

        other => panic!(
            "constant {:?} with type {:?} should have been eliminated by type inference",
            constant, other
        ),
    }
}
