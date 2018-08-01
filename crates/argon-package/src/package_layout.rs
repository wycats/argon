use crate::manifest::PackageDetails;
use crate::read_manifest;
use failure::Error;
use std::path::Path;

pub struct PackageLayout {
    pub name: String,
    pub manifest: Box<Path>,
    pub lib: Box<Path>,
    pub root: Box<Path>,
    pub out: Box<Path>,
}

pub fn package_layout(filename: &str) -> Result<PackageLayout, Error> {
    let PackageDetails { root, manifest } = read_manifest(filename)?;

    let layout = PackageLayout {
        name: manifest.project.name.clone(),
        root: root.to_owned(),
        manifest: root.join("Argon.toml").into_boxed_path(),
        lib: root.join("src").join("lib.argon").into_boxed_path(),
        out: root.join("out").into_boxed_path(),
    };

    Ok(layout)
}
