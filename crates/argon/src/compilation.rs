use crate::prelude::*;

use crate::database::Arcish;
use crate::database::*;
use parity_wasm::elements;

pub struct Database {
    leaves: Leaves,
    tables: Arc<Tables>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            leaves: Leaves::new(),
            tables: Arc::new(Tables::new()),
        }
    }

    pub fn shared(&'db self) -> SharedDatabase<'db> {
        SharedDatabase {
            leaves: &self.leaves,
            tables: self.tables.clone(),
        }
    }

    pub fn codemap(&self) -> &codespan::CodeMap {
        &self.leaves.files
    }

    pub fn add(&mut self, name: AbsolutePath, src: String) -> Result<(), ArgonError> {
        let filename = codespan::FileName::real(name.as_ref());
        let filemap = self.leaves.files.add_filemap(filename, src);
        self.leaves
            .code
            .insert_shared(name, VersionedCell::from_arcish(Arcish::weak(&filemap)));

        Ok(())
    }

    pub fn add_file(&mut self, name: AbsolutePath) -> Result<(), ArgonError> {
        let mut src = String::new();
        let mut file = File::open(name.as_ref())?;
        file.read_to_string(&mut src)?;

        let filename = codespan::FileName::real(name.as_ref());
        let filemap = self.leaves.files.add_filemap(filename, src);
        self.leaves
            .code
            .insert_shared(name, VersionedCell::from_arcish(Arcish::weak(&filemap)));

        Ok(())
    }
}

pub struct Leaves {
    crate files: codespan::CodeMap,
    crate code: Table<AbsolutePath, codespan::FileMap>,
}

impl Leaves {
    crate fn new() -> Leaves {
        Leaves {
            files: codespan::CodeMap::new(),
            code: Table::new(),
        }
    }
}

pub struct Tables {
    crate ast: AstTable,
    crate typed: TypedTable,
    crate wasm: WasmTable,
}

impl Tables {
    fn new() -> Tables {
        Tables {
            ast: AstTable::new(),
            typed: TypedTable::new(),
            wasm: WasmTable::new(),
        }
    }

    pub fn ast(&self) -> &AstTable {
        &self.ast
    }

    pub fn typed(&self) -> &TypedTable {
        &self.typed
    }

    pub fn wasm(&self) -> &WasmTable {
        &self.wasm
    }
}

pub struct SharedDatabase<'a> {
    leaves: &'a Leaves,
    tables: Arc<Tables>,
}

impl SharedDatabase<'a> {
    crate fn clone(&self) -> SharedDatabase<'a> {
        SharedDatabase {
            leaves: self.leaves,
            tables: self.tables.clone(),
        }
    }

    crate fn tables(&self) -> Arc<Tables> {
        self.tables.clone()
    }

    crate fn get_file(&self, name: &AbsolutePath) -> Option<VersionedCell<codespan::FileMap>> {
        self.leaves.code.get(name)
    }
}

pub struct Compilation<'db> {
    database: SharedDatabase<'db>,
}

impl Compilation<'db> {
    pub fn new(database: SharedDatabase<'db>) -> Compilation<'_> {
        Compilation { database }
    }

    pub fn get(
        &mut self,
        path: &AbsolutePath,
    ) -> GetResult<VersionedCell<elements::Module>, ArgonError>
    where
        ArgonError: 'static,
    {
        self.database
            .tables()
            .wasm()
            .get(self.database.clone(), path)
    }
}
