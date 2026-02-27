use collection_cap::CapConstraint;
use collection_cap::err::{CapError, CapOverflow, CapUnderflow};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

type TestArray = [i32; CAP];

const TARGET_OVERFLOW: CapOverflow<TestArray> = CapOverflow::new(CAP + 1);
const TARGET_UNDERFLOW: CapUnderflow<TestArray> = CapUnderflow::new(CAP - 1);

check_eq!(cap_constraint: TestArray::check_if_can_fit(&FITS_ITER) => Ok(()));
check_eq!(cap_constraint_overflow: TestArray::check_if_can_fit(&OVER_ITER) 
    => Err(CapError::Overflow(TARGET_OVERFLOW)));
check_eq!(cap_constraint_underflow: TestArray::check_if_can_fit(&UNDER_ITER) 
    => Err(CapError::Underflow(TARGET_UNDERFLOW)));

panics!(bad_iter: TestArray::check_if_can_fit(&INVALID_ITERATOR) 
    => "Invalid size hint: InvalidSizeHint");
