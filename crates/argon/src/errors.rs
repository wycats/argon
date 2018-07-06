use crate::prelude::*;

crate mod compile_error;
crate mod error;
crate mod unify;

#[derive(Debug)]
pub enum ArgonError {
    Error(failure::Error),
    CompileError(CompileError),
    Prototype(String),
}

impl ArgonError {
    crate fn bail<T>(value: impl Into<String>) -> Result<T, ArgonError> {
        Err(ArgonError::Prototype(value.into()))
    }
}

impl From<CompileError> for ArgonError {
    fn from(error: CompileError) -> ArgonError {
        ArgonError::CompileError(error)
    }
}

impl From<failure::Error> for ArgonError {
    fn from(error: failure::Error) -> ArgonError {
        ArgonError::Error(error)
    }
}

impl From<codespan::SpanError> for ArgonError {
    fn from(error: codespan::SpanError) -> ArgonError {
        ArgonError::Error(failure::Error::from(error))
    }
}

impl From<std::io::Error> for ArgonError {
    fn from(error: std::io::Error) -> ArgonError {
        ArgonError::Error(failure::Error::from(error))
    }
}

impl From<failure::Context<String>> for ArgonError {
    fn from(error: failure::Context<String>) -> ArgonError {
        ArgonError::Error(failure::Error::from(error))
    }
}
