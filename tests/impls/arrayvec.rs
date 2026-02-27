use collection_cap::err::CapOverflow;
use collection_cap::{CapConstraint, RemainingCap};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

use arrayvec::ArrayVec;

type TestArrayVec = ArrayVec<i32, CAP>;

const TARGET_OVERFLOW: CapOverflow<TestArrayVec> = CapOverflow::new(CAP + 1);

check_eq!(remaining_cap: RemainingCap::remaining_cap(&TestArrayVec::new()) => CAP);
check_eq!(cap_constraint: TestArrayVec::check_if_can_fit(&FITS_ITER) => Ok(()));
check_eq!(cap_constraint_overflow: TestArrayVec::check_if_can_fit(&OVER_ITER) 
    => Err(TARGET_OVERFLOW));

panics!(bad_iter: TestArrayVec::check_if_can_fit(&INVALID_ITERATOR) 
    => "Invalid size hint: InvalidSizeHint");
