#[macro_use]
crate mod table;
crate mod absolute_path;
crate mod ast_table;
crate mod typed_table;
crate mod versioned_cell;
crate mod wasm_table;

crate use self::absolute_path::AbsolutePath;
crate use self::ast_table::AstTable;
crate use self::table::Table;
pub use self::table::{GetResult, SkipResult};
crate use self::typed_table::TypedTable;
crate use self::versioned_cell::derive_from_revision;
pub use self::versioned_cell::{bump, derive_from, revision, Arcish, VersionedCell};
crate use self::wasm_table::WasmTable;
