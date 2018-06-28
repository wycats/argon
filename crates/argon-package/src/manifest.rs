#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    name: String,
    license: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    project: Project,
}
