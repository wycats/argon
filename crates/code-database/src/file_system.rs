use crate::path::AbsolutePath;
use failure::Error;
use std::io::prelude::*;

pub trait FileSystem {
    type File: File;
    fn read_file(&mut self, path: AbsolutePath) -> Result<Self::File, Error>;
}

pub trait File {
    type Read: Read + Seek + Send + Sync;

    fn path(&self) -> &AbsolutePath;
    fn revision(&mut self) -> i64;
    fn valid_for(&mut self, snapshot: i64) -> bool;
    fn read(&mut self) -> Result<Self::Read, Error>;
}
