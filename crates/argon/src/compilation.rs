use codespan::{CodeMap, FileName};
use crate::file_system::fs::File;
use crate::file_system::path::AbsolutePath;
use crate::FileSystem;
use failure::Error;
use std::path::PathBuf;

#[derive(new)]
pub struct Compilation<FS: FileSystem> {
    file_system: FS,
    map: CodeMap,
}

impl<FS: FileSystem> Compilation<FS> {
    pub fn add(&mut self, path: impl AsRef<str>) -> Result<(), Error> {
        let path = AbsolutePath::expand(path.as_ref())?;
        let mut file = self.file_system.read_file(path)?;
        let mut read = file.read()?;

        let mut content = String::new();
        read.read_to_string(&mut content);
        self.map.add_filemap(FileName::Real(path), content);

        unimplemented!()
    }
}
