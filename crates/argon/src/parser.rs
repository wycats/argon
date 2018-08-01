use crate::lexer::{Lexer, Token};
use crate::{ast, CompileError, ModuleParser};

pub type LalrpopParseError = lalrpop_util::ParseError<usize, Token, ()>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorLocation {
    EOF,
    Byte(usize),
}

pub fn parse(source: &'input str, codespan_start: usize) -> Result<ast::Module, CompileError> {
    ModuleParser::new()
        .parse(Lexer::new(source.as_ref(), codespan_start))
        .map_err(|e| CompileError::ParseError(e))
}

pub fn location(error: CompileError) -> ErrorLocation {
    match error {
        CompileError::ParseError(err) => match err {
            lalrpop_util::ParseError::InvalidToken { location } => ErrorLocation::Byte(location),
            lalrpop_util::ParseError::UnrecognizedToken { token: None, .. } => ErrorLocation::EOF,
            lalrpop_util::ParseError::UnrecognizedToken {
                token: Some((location, ..)),
                ..
            } => ErrorLocation::Byte(location),
            lalrpop_util::ParseError::ExtraToken {
                token: (location, ..),
                ..
            } => ErrorLocation::Byte(location),
            lalrpop_util::ParseError::User { error } => panic!("{:?}", error),
        },

        other => panic!("Cannot get location from {}", other),
    }
}
