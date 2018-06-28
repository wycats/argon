use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub license: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub project: Project,
}

#[derive(Debug, Serialize)]
pub struct PackageDetails {
    pub manifest: Manifest,
    pub root: Box<Path>,
}
