mod array;
mod slice;

#[cfg(feature = "arrayvec")]
mod arrayvec;

#[cfg(feature = "alloc")]
pub mod alloc_impl;
