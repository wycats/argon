use crate::prelude::*;

crate use super::constraint_set::Constraints;
use codespan::ByteSpan;
use crate::ir::InferType;
use language_reporting::{Diagnostic, Label};

#[cfg(test)]
mod tests;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
crate struct Constraint {
    crate left: InferType,
    crate right: InferType,
    crate why: Why,
}

impl Constraint {
    crate fn new<T>(left: InferType, right: InferType, why: Why) -> Constraint {
        Constraint { left, right, why }
    }

    crate fn double(left: InferType, right: InferType) -> Constraint {
        let left_span = left.span();
        let right_span = right.span();

        Constraint {
            left,
            right,
            why: Why::Unimplemented(left_span, right_span),
        }
    }
}

impl std::ops::Add for Constraint {
    type Output = Constraints;

    fn add(self, rhs: Constraint) -> Constraints {
        Constraints(self) + rhs
    }
}

#[allow(non_snake_case)]
crate fn Constraint(left: InferType, right: InferType, why: Why) -> Constraint {
    Constraint { left, right, why }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum Why {
    Binary {
        left: ByteSpan,
        right: ByteSpan,
        expr: ByteSpan,
    },

    Unimplemented(ByteSpan, ByteSpan),
}

impl ToDiagnostic for Why {
    fn to_diagnostic(&self) -> language_reporting::Diagnostic {
        match self {
            Why::Binary { left, right, expr } => {
                let left_label = Label::new_primary(*left).with_message("this");
                let right_label = Label::new_primary(*right).with_message("this");

                Diagnostic::new_error("Type Error")
                    .with_label(left_label)
                    .with_label(right_label)
            }

            Why::Unimplemented(left, right) => {
                let left_label = Label::new_primary(*left).with_message("this");
                let right_label = Label::new_primary(*right).with_message("this");

                Diagnostic::new_error("Type Error (TODO)")
                    .with_label(left_label)
                    .with_label(right_label)
            }
        }
    }
}
