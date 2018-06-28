#![feature(box_syntax, box_pattern)]

#[macro_use]
extern crate serde_derive;

mod error;
mod manifest;
mod package_layout;
mod parse;
mod read_manifest;

pub use crate::error::PackageError;
pub use crate::manifest::{Manifest, Project};
pub use crate::package_layout::package_layout;
pub use crate::parse::*;
pub use crate::read_manifest::read_manifest;
