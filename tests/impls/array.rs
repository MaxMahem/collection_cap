use collection_cap::StaticCap;
use collection_cap::cap::StaticExactCap;

use crate::common::check_eq;
use crate::common::consts::CAP;

check_eq!(capacity: <[i32; CAP]>::CAP => StaticExactCap::<CAP>);
