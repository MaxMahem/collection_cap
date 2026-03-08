#[path = "../common/mod.rs"]
mod common;

pub(crate) mod macros;

pub(crate) use macros::*;

mod consts;
mod unbounded;
mod val;
