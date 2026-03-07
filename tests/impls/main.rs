#[path = "../common/mod.rs"]
mod common;

mod array;
mod arrayvec;
mod range;
mod slice;

#[cfg(feature = "alloc")]
mod alloc_impl;
