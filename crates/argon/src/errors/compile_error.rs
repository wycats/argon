use crate::prelude::*;

use codespan_reporting::{Diagnostic, Label};
use crate::compile::MathOperator;
use crate::ir::resolved::ResolveError;
use crate::ir::{InferType, Type};
use crate::parser::LalrpopParseError;

#[derive(Debug, PartialEq, Clone)]
pub enum CompileError {
    ParseError(LalrpopParseError),
    ResolveError(ResolveError),
    TypeError(TypeError),
    UnifyError(InferType, InferType),
    LexError,
    Unimplemented,
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ToDiagnostic for CompileError {
    fn to_diagnostic(&self) -> codespan_reporting::Diagnostic {
        match self {
            CompileError::UnifyError(left, right) => {
                let left_label = Label::new_primary(left.span()).with_message("this");
                let right_label = Label::new_primary(right.span()).with_message("this");

                Diagnostic::new_error("Type Error")
                    .with_label(left_label)
                    .with_label(right_label)
            }

            other => unimplemented!("Diagnostics for {:#?}", other),
        }
    }
}

impl Fail for CompileError {}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum TypeError {
    MismatchedBinary(MathOperator, Type, Type),
}

impl From<ResolveError> for CompileError {
    fn from(error: ResolveError) -> CompileError {
        CompileError::ResolveError(error)
    }
}

impl From<TypeError> for CompileError {
    fn from(error: TypeError) -> CompileError {
        CompileError::TypeError(error)
    }
}

pub trait ToDiagnostic {
    fn to_diagnostic(&self) -> codespan_reporting::Diagnostic;
}
