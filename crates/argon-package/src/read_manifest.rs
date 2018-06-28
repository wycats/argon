use crate::manifest::Manifest;
use failure::Error;
use failure::ResultExt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::{fs, path};

pub fn read_manifest(filename: &str) -> Result<Manifest, Error> {
    let expanded = shellexpand::full(filename).with_context(|_| "shellexpand".to_string())?;
    let canonical = fs::canonicalize(path::Path::new(expanded.as_ref())).with_context(|_| {
        format!(
            "Attempting to canonicalize {:?} (original was {:?})",
            expanded, filename
        )
    })?;

    let manifest_path = Path::new(&canonical).join("Argon.toml");

    let mut manifest = File::open(&manifest_path)
        .with_context(|_| format!("No Argon.toml found at {}", manifest_path.display()))?;

    let mut contents = String::new();
    manifest
        .read_to_string(&mut contents)
        .with_context(|_| format!("Failed to read from Argon.toml"))?;

    Ok(crate::parse(&contents)?)
}
