use crate::entry::Entry;
use crate::file_system::file_entry::FileEntry;
use crate::file_system::real_file::RealFile;
use crate::file_system::watch::Watch;
use crate::LeafTable;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
pub struct TransactionId(crate usize);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
pub struct TableId(usize);

pub struct Database {
    transaction_count: usize,
    table_count: usize,
    watch: Watch,
    file_table: LeafTable<FileEntry<RealFile>>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            transaction_count: 0,
            table_count: 1,
            watch: Watch::new(),
            file_table: LeafTable::new(TableId(0)),
        }
    }

    pub fn begin(&mut self) -> TransactionId {
        let transaction_count = self.transaction_count;
        self.transaction_count += 1;
        TransactionId(transaction_count)
    }

    pub fn commit(&mut self) {
        // TODO: Sweep
    }

    pub fn watch_mut(&mut self) -> &mut Watch {
        &mut self.watch
    }

    pub fn watch(&self) -> &Watch {
        &self.watch
    }

    pub fn files(&self) -> &LeafTable<FileEntry<RealFile>> {
        &self.file_table
    }

    pub fn files_mut(&mut self) -> &mut LeafTable<FileEntry<RealFile>> {
        &mut self.file_table
    }

    pub fn new_table<E: Entry>(&mut self, f: impl FnOnce(TableId) -> LeafTable<E>) -> LeafTable<E> {
        let table_count = self.table_count;
        self.table_count += 1;
        f(TableId(table_count))
    }
}
