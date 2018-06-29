use failure::{Error, ResultExt};
use std::fs;
use std::path::{Path, PathBuf};

pub struct AbsolutePath(PathBuf);

impl AbsolutePath {
    pub fn as_path(&self) -> &Path {
        self.0.as_path()
    }

    pub fn expand(filename: impl AsRef<str>) -> Result<AbsolutePath, Error> {
        let filename = filename.as_ref();
        let expanded = shellexpand::full(filename).with_context(|_| "shellexpand".to_string())?;

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
            Err(crate::DatabaseError::NotImplemented("filename != expanded"))?;
        }

        let canonical = fs::canonicalize(expanded.as_ref()).with_context(|_| {
            format!(
                "Attempting to canonicalize {:?} (original was {:?})",
                expanded, filename
            )
        })?;

        if filename != canonical.to_str().unwrap() {
            Err(crate::DatabaseError::NotImplemented(
                "filename != canonical",
            ))?;
        }

        Ok(AbsolutePath(canonical))
    }
}
