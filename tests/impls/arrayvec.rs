use collection_cap::StaticCap;
use collection_cap::err::CapOverflow;

use crate::common::consts::*;
use crate::common::{check_eq, panics};

use arrayvec::ArrayVec;

type TestArrayVec = ArrayVec<i32, CAP>;

const TARGET_OVERFLOW: CapOverflow<TestArrayVec> = CapOverflow::new(CAP + 1);

check_eq!(cap_constraint: TestArrayVec::check_compatability(&COMPAT_ITER) => Ok(()));
check_eq!(cap_constraint_overflow: TestArrayVec::check_compatability(&OVER_ITER) 
    => Err(TARGET_OVERFLOW));

panics!(bad_iter: TestArrayVec::check_compatability(&INVALID_ITER) 
    => "Invalid size hint");
