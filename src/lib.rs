#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic, clippy::cargo, clippy::nursery)]
#![warn(missing_docs, missing_debug_implementations)]
#![allow(clippy::match_bool, clippy::single_match_else)]
#![no_std]
#![forbid(unsafe_code)]

pub(crate) const EMPTY_RANGE_MSG: &str = "Range must not be empty";
pub(crate) const INVALID_RANGE_MSG: &str = "Invalid range (start > end)";
pub(crate) mod iter;
pub(crate) use iter::IterExt;

mod capacity;
mod impls;
mod iter_cap_ext;

pub use capacity::*;
pub use iter_cap_ext::*;

/// Capacity constraints.
pub mod cap;
/// Capacity validation errors.
pub mod err;
