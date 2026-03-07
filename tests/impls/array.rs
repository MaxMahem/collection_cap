#![allow(unused_imports)]
use crate::common::consts::*;
use collection_cap::StaticCap;
use collection_cap::cap::StaticExactCap;

use crate::common::check_eq;

check_eq!(capacity: <[i32; CAP]>::CAP => StaticExactCap::<CAP>);
