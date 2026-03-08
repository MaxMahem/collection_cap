#![allow(unused_imports)]
use crate::common::consts::*;
use collection_cap::ConstCap;
use collection_cap::cap::ConstExactCap;

use crate::common::check_eq;

check_eq!(capacity: <[i32; CAP]>::CAP => ConstExactCap::<CAP>);
