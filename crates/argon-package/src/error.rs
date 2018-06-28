use failure::Fail;
use std::fmt;

#[derive(Debug)]
pub enum PackageError {
    ParseError(String),
    SerializeError(serde_json::error::Error),
}

impl fmt::Display for PackageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PackageError::ParseError(desc) => write!(f, "Parse Error: {}", desc),
            PackageError::SerializeError(desc) => write!(f, "Serialize Error: {}", desc),
        }
    }
}

impl Fail for PackageError {}
