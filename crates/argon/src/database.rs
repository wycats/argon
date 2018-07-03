crate mod ast_table;
crate mod code_table;
crate mod codemap_table;
crate mod map_table_trait;
crate mod parse;
crate mod typed_table;
crate mod wasm_table;

crate use self::ast_table::AstTable;
crate use self::code_table::CodeTable;
crate use self::map_table_trait::MapTableTrait;
crate use self::typed_table::TypedTable;
crate use self::wasm_table::WasmTable;
