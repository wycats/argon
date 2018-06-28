use crate::{Manifest, PackageError};

pub fn parse(input: &str) -> Result<Manifest, toml::de::Error> {
    toml::from_str(input)
}

pub fn to_json(input: &Manifest) -> Result<String, PackageError> {
    serde_json::to_string(input).map_err(|e| PackageError::SerializeError(e))
}

pub fn to_json_pretty(input: &Manifest) -> Result<String, PackageError> {
    serde_json::to_string_pretty(input).map_err(|e| PackageError::SerializeError(e))
}
