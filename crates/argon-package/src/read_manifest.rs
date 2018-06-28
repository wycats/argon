use crate::manifest::PackageDetails;
use failure::Error;
use failure::ResultExt;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn expand_path(filename: &str) -> Result<Box<Path>, Error> {
    let expanded = shellexpand::full(filename).with_context(|_| "shellexpand".to_string())?;

    let canonical = fs::canonicalize(expanded.as_ref()).with_context(|_| {
        format!(
            "Attempting to canonicalize {:?} (original was {:?})",
            expanded, filename
        )
    })?;

    Ok(canonical.into_boxed_path())
}

pub fn read_manifest(filename: &str) -> Result<PackageDetails, Error> {
    let root_path = expand_path(filename)?;
    let manifest_path = root_path.join("Argon.toml");

    let mut manifest = File::open(&manifest_path)
        .with_context(|_| format!("No Argon.toml found at {}", manifest_path.display()))?;

    let mut contents = String::new();
    manifest
        .read_to_string(&mut contents)
        .with_context(|_| format!("Failed to read from Argon.toml"))?;

    let manifest = crate::parse(&contents)?;

    Ok(PackageDetails {
        root: root_path,
        manifest,
    })
}
