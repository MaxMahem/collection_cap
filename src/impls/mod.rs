mod array;
mod range;
mod slice;

#[cfg(feature = "arrayvec")]
mod arrayvec;

#[cfg(feature = "alloc")]
pub mod alloc_impl;
