#![feature(in_band_lifetimes, box_patterns, box_syntax)]
#![warn(rust_2018_idioms)]

pub mod collections;
pub mod constraints;
pub mod errors;
pub mod solution;
pub mod types;
pub mod unify;

#[cfg(test)]
mod test;

pub use crate::constraints::*;
pub use crate::errors::*;
pub use crate::solution::*;
pub use crate::types::*;

#[cfg(test)]
mod ena_test;
