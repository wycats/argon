use super::path::AbsolutePath;
use super::watch::WatchKey;
use crate::file_system::file_entry::FileEntry;
use crate::file_system::File;
use failure::Error;

pub enum Watching {
    Unwatched,
    Watched(WatchKey),
}

#[derive(new)]
pub struct RealFile {
    path: AbsolutePath,
    watch_status: Watching,
}

impl RealFile {
    pub fn unwatched(path: impl Into<AbsolutePath>) -> RealFile {
        RealFile {
            path: path.into(),
            watch_status: Watching::Unwatched,
        }
    }

    pub fn into_entry(self) -> FileEntry<RealFile> {
        FileEntry::new(self.path.clone(), self)
    }
}

impl File for RealFile {
    type Read = std::fs::File;

    fn path(&self) -> &AbsolutePath {
        &self.path
    }

    fn read(&mut self) -> Result<Self::Read, Error> {
        Ok(std::fs::File::open(&self.path)?)
    }
}
