use crate::lexer::{Lexer, Tok};
use crate::{ast, CompileError, ModuleParser};

pub type ParseError<'input> = lalrpop_util::ParseError<usize, Tok<'static>, CompileError>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorLocation {
    EOF,
    Byte(usize),
}

pub fn parse(source: &'input str) -> Result<ast::Module<'input>, ParseError<'static>> {
    ModuleParser::new()
        .parse(Lexer::new(source.as_ref()))
        .map_err(|e| e.map_token(|t| t.into_owned()))
}

pub fn location(error: ParseError) -> ErrorLocation {
    match error {
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
    }
}
