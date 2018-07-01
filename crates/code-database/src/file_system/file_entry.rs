use super::path::AbsolutePath;
use crate::database::TransactionId;
use crate::entry::Entry;
use crate::file_system;
use crate::tag::{DirtyableTag, Tag};

#[derive(Debug, new)]
pub struct FileEntry<F: file_system::File> {
    key: AbsolutePath,
    file: F,
}

impl<F: file_system::File> Entry for FileEntry<F> {
    type Key = AbsolutePath;
    type Tag = DirtyableTag;
    type Value = F;

    fn tag(&self) -> DirtyableTag {
        DirtyableTag::new()
    }

    fn key(&self) -> &AbsolutePath {
        &self.key
    }

    fn value(&self, _transaction: TransactionId) -> &F {
        &self.file
    }

    fn value_mut(&mut self, _transaction: TransactionId) -> &mut F {
        &mut self.file
    }
}

pub struct FileTag;

impl Tag for FileTag {
    fn revision(&self) -> usize {
        0
    }

    fn is_valid(&self, _snapshot: usize) -> bool {
        true
    }
}
