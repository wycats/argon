use crate::entry::Entry;
use crate::file_system;
use crate::tag::Tag;
use std::path::PathBuf;

impl<F: file_system::File> Entry for F {
    type Tag = FileTag;
    type Value = F;

    fn tag(&self) -> FileTag {
        FileTag {
            path: self.path().as_path().to_path_buf(),
        }
    }

    fn value(&self) -> &F {
        self
    }
}

pub struct FileTag {
    path: PathBuf,
}

impl Tag for FileTag {
    fn revision(&self) -> u64 {
        0
    }

    fn is_valid(&self, snapshot: u64) -> bool {
        true
    }
}
