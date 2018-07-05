use failure::{Error, ResultExt};
use std::borrow::Borrow;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AbsolutePath(PathBuf);

impl AbsolutePath {
    pub fn as_path(&self) -> &Path {
        self.0.as_path()
    }

    pub fn as_path_buf(&self) -> &PathBuf {
        &self.0
    }

    pub fn expand(filename: impl AsRef<Path>) -> Result<AbsolutePath, Error> {
        let filename = filename.as_ref();
        let filename_cow = filename.to_string_lossy();
        let filename_str: &str = filename_cow.borrow();

        let expanded = shellexpand::full(filename_str).with_context(|_| "shellexpand".to_string())?;

        let canonical = fs::canonicalize(expanded.as_ref()).with_context(|_| {
            format!(
                "Attempting to canonicalize {:?} (original was {:?})",
                expanded, filename
            )
        })?;

        Ok(AbsolutePath(canonical))
    }

    pub fn from_canonical(filename: impl AsRef<str>) -> Result<AbsolutePath, Error> {
        let filename = filename.as_ref();
        let expanded = shellexpand::full(filename).with_context(|_| "shellexpand".to_string())?;

        if filename != expanded {
            bail!("filename != expanded")
        }

        let canonical = fs::canonicalize(expanded.as_ref()).with_context(|_| {
            format!(
                "Attempting to canonicalize {:?} (original was {:?})",
                expanded, filename
            )
        })?;

        if filename != canonical.to_str().unwrap() {
            bail!("filename != canonical")
        }

        Ok(AbsolutePath(canonical))
    }
}

impl AsRef<Path> for AbsolutePath {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}
