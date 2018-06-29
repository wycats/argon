use failure::Fail;
use std::fmt;

#[derive(Debug)]
pub enum DatabaseError {
    NotImplemented(&'static str),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Fail for DatabaseError {}
