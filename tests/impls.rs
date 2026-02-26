mod common;

use collection_cap::err::{TargetCapError, TargetOverflow, TargetUnderflow};
use collection_cap::{CapConstraint, RemainingCap};

use common::consts::*;
use common::{check_eq, panics};

mod array_vec {
    use super::*;

    use arrayvec::ArrayVec;

    type TestArrayVec = ArrayVec<i32, CAP>;

    const TARGET_OVERFLOW: TargetOverflow<TestArrayVec> = TargetOverflow::new(CAP + 1);

    check_eq!(remaining_cap: RemainingCap::remaining_cap(&TestArrayVec::new()) => CAP);
    check_eq!(cap_constraint: TestArrayVec::check_if_can_fit(&FITS_ITER) => Ok(()));
    check_eq!(cap_constraint_overflow: TestArrayVec::check_if_can_fit(&OVER_ITER) 
        => Err(TARGET_OVERFLOW));

    panics!(bad_iter: TestArrayVec::check_if_can_fit(&INVALID_ITERATOR) 
        => "Invalid size hint: InvalidSizeHint");
}

mod array {
    use super::*;

    type TestArray = [i32; CAP];

    const TARGET_OVERFLOW: TargetOverflow<TestArray> = TargetOverflow::new(CAP + 1);
    const TARGET_UNDERFLOW: TargetUnderflow<TestArray> = TargetUnderflow::new(CAP - 1);

    check_eq!(cap_constraint: TestArray::check_if_can_fit(&FITS_ITER) => Ok(()));
    check_eq!(cap_constraint_overflow: TestArray::check_if_can_fit(&OVER_ITER) 
        => Err(TargetCapError::Overflow(TARGET_OVERFLOW)));
    check_eq!(cap_constraint_underflow: TestArray::check_if_can_fit(&UNDER_ITER) 
        => Err(TargetCapError::Underflow(TARGET_UNDERFLOW)));

    panics!(bad_iter: TestArray::check_if_can_fit(&INVALID_ITERATOR) 
        => "Invalid size hint: InvalidSizeHint");
}
