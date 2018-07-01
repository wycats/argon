pub mod file_entry;
pub mod path;
pub mod real_file;
pub mod watch;

use self::path::AbsolutePath;
use failure::Error;
use std::io::prelude::*;

pub trait FileSystem {
    type File: File;
    fn read_file(&mut self, path: AbsolutePath) -> Result<Self::File, Error>;
}

pub trait File: std::fmt::Debug {
    type Read: Read + Seek + Send + Sync;

    fn path(&self) -> &AbsolutePath;
    fn read(&mut self) -> Result<Self::Read, Error>;
}
