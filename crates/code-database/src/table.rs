mod leaf;
mod mapped;
mod table_trait;

#[cfg(test)]
mod tests;

pub use self::leaf::{EntryId, LeafTable};
pub use self::mapped::map;
pub use self::table_trait::Table;
