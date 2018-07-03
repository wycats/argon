use self::codemap_table::CodemapTable;
use code_database::{AbsolutePath, Database as CodeDatabase, RealFile};
use crate::database::*;
use failure::Error;
use parity_wasm::elements;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::path::Path;

pub struct Database {
    crate files: CodeDatabase,
    crate code: CodeTable,
    crate ast: AstTable,
    crate typed: TypedTable,
    crate wasm: WasmTable,
}

impl Database {
    crate fn without_wasm(&mut self) -> (&mut WasmTable, DatabaseWithoutWasm) {
        let db = DatabaseWithoutWasm {
            files: &mut self.files,
            code: &mut self.code,
            ast: &mut self.ast,
            typed: &mut self.typed,
        };

        (&mut self.wasm, db)
    }
}

pub struct DatabaseWithoutWasm<'parent> {
    crate files: &'parent mut CodeDatabase,
    crate code: &'parent mut CodeTable,
    crate ast: &'parent mut AstTable,
    crate typed: &'parent mut TypedTable,
}

impl DatabaseWithoutWasm<'parent> {
    crate fn without_typed(&mut self) -> (&mut TypedTable, DatabaseWithoutTyped<'_>) {
        let db = DatabaseWithoutTyped {
            files: self.files,
            code: self.code,
            ast: self.ast,
        };

        (self.typed, db)
    }
}

pub struct DatabaseWithoutTyped<'parent> {
    crate files: &'parent mut CodeDatabase,
    crate code: &'parent mut CodeTable,
    crate ast: &'parent mut AstTable,
}

impl DatabaseWithoutTyped<'parent> {
    crate fn parts(&mut self) -> (CodemapTable, &mut AstTable) {
        let code = CodemapTable {
            files: self.files,
            code: self.code,
        };

        (code, self.ast)
    }

    crate fn code(&mut self) -> CodemapTable {
        CodemapTable {
            files: self.files,
            code: self.code,
        }
    }
}

pub struct Compilation {
    database: Database,
}

impl Compilation {
    fn db(&mut self) -> &mut Database {
        &mut self.database
    }

    pub fn new() -> Compilation {
        Compilation {
            database: Database {
                files: CodeDatabase::new(),
                code: CodeTable::new(),
                ast: AstTable::new(),
                typed: TypedTable::new(),
                wasm: WasmTable::new(),
            },
        }
    }

    pub fn add(&mut self, path: impl AsRef<Path>) -> Result<AbsolutePath, Error> {
        let db = &mut self.database.files;
        let txn = db.begin();
        let path = AbsolutePath::expand(path)?;
        let file = RealFile::unwatched(path.clone());
        db.files_mut().add_entry(file.into_entry(), txn);
        db.commit();

        Ok(path)
    }

    pub fn get(&mut self, path: &AbsolutePath) -> Result<Option<Cow<elements::Module>>, Error>
    where
        Error: 'static,
    {
        let txn = {
            let db = &mut self.database.files;
            trace!(target: "argon::compilation", "files: {:#?}", db.files_mut());
            db.begin()
        };

        let db = self.db();
        let (wasm, mut rest) = db.without_wasm();
        wasm.get(&mut rest, path, txn)
    }
}
