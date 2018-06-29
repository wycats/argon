// A database vends Tags and manages transactions

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
pub struct TransactionId(crate usize);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
pub struct TableId(usize);

#[derive(new)]
pub struct Database {
    #[new(default)]
    transaction_count: usize,

    #[new(default)]
    table_count: usize,
}

impl Database {
    pub fn begin(&mut self) -> TransactionId {
        let transaction_count = self.transaction_count;
        self.transaction_count += 1;
        TransactionId(transaction_count)
    }

    pub fn commit(&mut self) {
        // TODO: Sweep
    }

    pub fn new_table(&mut self) -> TableId {
        let table_count = self.table_count;
        self.table_count += 1;
        TableId(table_count)
    }
}
