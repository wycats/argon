use super::start;
use crate::ir::resolved;
use crate::Type;

pub struct Resolve<'input> {
    module: start::Module<'input>,
    narrowed: usize,
}

impl start::Expression {
    fn attempt_resolve(self, locals: &[Type]) -> start::Expression {
        let start::Expression { expression, types } = self;

        match expression {
            start::ExpressionEnum::Const(_) => return start::Expression { expression, types },
            start::ExpressionEnum::VariableAccess(local) => {
                let ty = locals[local as usize];
                start::Expression {
                    expression: start::ExpressionEnum::VariableAccess(local),
                    types: start::PossibleTypes::resolved(ty),
                }
            }
            start::ExpressionEnum::Binary(operator, box binary) => {
                let start::BinaryExpression { lhs, rhs } = binary;
                let types = lhs.types.clone().intersect(rhs.types.clone()).0;

                let binary = start::BinaryExpression { lhs, rhs };

                let expression = start::ExpressionEnum::Binary(operator, Box::new(binary));

                start::Expression { expression, types }
            }
        }
    }
}
