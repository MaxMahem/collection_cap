#[path = "../common/mod.rs"]
mod common;

mod array;
mod arrayvec;
mod slice;

#[cfg(feature = "alloc")]
mod alloc_impl;
