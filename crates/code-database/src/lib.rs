#![feature(try_trait)]

#[macro_use]
extern crate derive_new;

pub mod database;
pub mod entry;
pub mod error;
pub mod file_system;
pub mod table;
pub mod tag;

pub use self::database::{Database, TransactionId};
pub use self::error::DatabaseError;
pub use self::file_system::path;
pub use self::file_system::path::AbsolutePath;
pub use self::file_system::real_file::RealFile;
pub use self::file_system::File as FileTrait;
pub use self::table::LeafTable;

pub type FileTable<F> = LeafTable<crate::file_system::file_entry::FileEntry<F>>;

pub mod prelude {
    pub use super::entry::Entry;
    pub use super::file_system::File as FileTrait;
}
