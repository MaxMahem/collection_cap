use collection_cap::cap::StaticMaxCap;
use collection_cap::{StaticCap, VariableCap};

use crate::common::check_eq;
use crate::common::consts::*;

use arrayvec::ArrayVec;

type TestArrayVec = ArrayVec<i32, CAP>;

check_eq!(variable_capacity: VariableCap::capacity(&TestArrayVec::new()) => MAX_CAP_VAL);
check_eq!(static_capacity: TestArrayVec::CAP => StaticMaxCap::<CAP>);
