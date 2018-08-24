use crate::prelude::*;

use crate::compile::MathOperator;
use crate::infer::constraint::Why;
use crate::ir::resolved::ResolveError;
use crate::ir::{InferType, Type};
use crate::parser::LalrpopParseError;

#[derive(Debug, PartialEq, Clone)]
pub enum CompileError {
    ParseError(LalrpopParseError),
    ResolveError(ResolveError),
    TypeError(TypeError),
    UnifyError(UnifyError),
    LexError,
    Unimplemented,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnifyError {
    pub left: InferType,
    pub right: InferType,
    pub why: Why,
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ToDiagnostic for CompileError {
    fn to_diagnostic(&self) -> language_reporting::Diagnostic {
        match self {
            CompileError::UnifyError(UnifyError { left, right, why }) => {
                let mut diag = why.to_diagnostic();
                diag.message = format!("Type Error: {} with {}", left, right);
                diag
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
    fn to_diagnostic(&self) -> language_reporting::Diagnostic;
}
