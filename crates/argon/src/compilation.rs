use code_database::{AbsolutePath, Database, RealFile};
use crate::database::*;
use failure::Error;
use parity_wasm::elements;
use std::path::Path;

pub struct Compilation {
    database: Database,
    code: CodeTable,
    ast: AstTable,
    typed: TypedTable,
    wasm: WasmTable,
}

impl Compilation {
    pub fn new() -> Compilation {
        Compilation {
            database: Database::new(),
            code: CodeTable::new(),
            ast: AstTable::new(),
            typed: TypedTable::new(),
            wasm: WasmTable::new(),
        }
    }

    pub fn add(&mut self, path: impl AsRef<Path>) -> Result<AbsolutePath, Error> {
        let db = &mut self.database;
        let txn = db.begin();
        let path = AbsolutePath::expand(path)?;
        let file = RealFile::unwatched(path.clone());
        db.files_mut().add_entry(file.into_entry(), txn);
        db.commit();

        Ok(path)
    }

    pub fn get(&mut self, path: &AbsolutePath) -> Result<Option<&elements::Module>, Error> {
        let db = &mut self.database;

        println!("{:#?}", db.files_mut());

        let txn = db.begin();
        let module = self.wasm.get(
            db.files_mut(),
            &mut self.code,
            &mut self.ast,
            &mut self.typed,
            path,
            txn,
        )?;
        println!("{:#?}", module);
        // let file = code.get(db.filLes_mut(), &path, txn)?;
        db.commit();

        // let path = AbsolutePath::expand(path.as_ref())?;
        // let mut file = self.file_system.read_file(path)?;
        // let mut read = file.read()?;

        // let mut content = String::new();
        // read.read_to_string(&mut content);
        // self.map.add_filemap(FileName::Real(path), content);

        Ok(module)
    }
}
