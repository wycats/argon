use code_database::{AbsolutePath, Database, FileTrait, RealFile};
use crate::database::code_table::CodeTable;
use failure::Error;
use std::io::prelude::*;

#[derive(new)]
pub struct Compilation {}

impl Compilation {
    pub fn add(&mut self, path: impl AsRef<str>) -> Result<(), Error> {
        let mut db = Database::new();
        let mut code = CodeTable::new();

        let txn = db.begin();
        let path = AbsolutePath::expand(path)?;
        let file = RealFile::unwatched(path.clone());
        db.files_mut().add_entry(file.into_entry(), txn);
        db.commit();

        let txn = db.begin();
        let file = code.get(db.files_mut(), &path, txn)?;
        db.commit();

        // let path = AbsolutePath::expand(path.as_ref())?;
        // let mut file = self.file_system.read_file(path)?;
        // let mut read = file.read()?;

        // let mut content = String::new();
        // read.read_to_string(&mut content);
        // self.map.add_filemap(FileName::Real(path), content);

        Ok(())
    }
}
