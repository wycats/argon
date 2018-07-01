use super::path::AbsolutePath;
use super::real_file::{RealFile, Watching};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct WatchKey(usize);

pub enum WatchUpdate {
    UpToDate,
    Added,
    Deleted,
}

pub enum WatchStatus {
    File,
    Directory,
    Missing,
}

pub enum WatchStrategy {
    Timestamp,
    ContentHash,
}

#[derive(new, Default)]
pub struct Watch {}

impl Watch {
    pub fn register(&mut self, _path: AbsolutePath, _strategy: WatchStrategy) -> WatchKey {
        unimplemented!()
    }

    pub fn status(&self, _key: WatchKey) -> WatchStatus {
        unimplemented!()
    }

    pub fn stop(&self, _key: WatchKey) {
        unimplemented!()
    }

    pub fn file(&mut self, path: impl Into<AbsolutePath>, strategy: WatchStrategy) -> RealFile {
        let path = path.into();

        let key = self.register(path.clone(), strategy);
        RealFile::new(path, Watching::Watched(key))
    }
}
