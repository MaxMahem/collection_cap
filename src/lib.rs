#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic, clippy::cargo, clippy::nursery)]
#![warn(missing_docs, missing_debug_implementations)]
#![allow(clippy::match_bool, clippy::single_match_else)]
#![no_std]
#![forbid(unsafe_code)]

mod capacity;
mod impls;
mod iter_cap_ext;
mod marker;

/// Capacity validation errors.
pub mod err;

pub use capacity::*;
pub use iter_cap_ext::*;
pub use marker::*;
