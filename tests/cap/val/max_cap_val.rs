pub use core::ops::Bound;

use crate::common::check_eq;
use crate::common::consts::*;
use crate::{caps, check_intersects, check_overlaps, contains_size, range_bounds};

use collection_cap::cap::{ConstMaxCap, MaxCapVal, UnboundedCap};
use collection_cap::err::{EmptyRange, MaxOverflow, MinOverflow};

const MAX_CAP_VAL: MaxCapVal = MaxCapVal(CAP);

check_eq!(zero: MaxCapVal::ZERO => MaxCapVal(0));
check_eq!(from_static: MaxCapVal::from(ConstMaxCap::<CAP>) => MAX_CAP_VAL);
check_eq!(from_range_to_inclusive: MaxCapVal::from(..=CAP) => MAX_CAP_VAL);

mod try_from_range_to {
    use super::*;

    check_eq!(valid: MaxCapVal::try_from(..CAP + 1) => Ok(MAX_CAP_VAL));
    check_eq!(empty: MaxCapVal::try_from(..0) => Err(EmptyRange));
}

check_eq!(capacity: MAX_CAP_VAL.capacity() => MAX_CAP_VAL);

caps!(MAX_CAP_VAL => { min: UnboundedCap, max: MAX_CAP_VAL });

contains_size!(MAX_CAP_VAL => { cap: true, under: true, over: false });

range_bounds!(MAX_CAP_VAL => { start: Bound::Unbounded, end: Bound::Included(&CAP) });

const MIN_OVERFLOW: MinOverflow<MaxCapVal> = MinOverflow::<MaxCapVal>::new(OVER_CAP, MAX_CAP_VAL);
check_intersects!(MAX_CAP_VAL => { overflow: Err(MIN_OVERFLOW), underflow: Ok(()) });

const MAX_OVERFLOW: MaxOverflow<MaxCapVal> = MaxOverflow::<MaxCapVal>::fixed(OVER_CAP, MAX_CAP_VAL);
check_overlaps!(MAX_CAP_VAL => {
    underflow: Ok(()),
    overflow: Err(MAX_OVERFLOW),
    unbounded: Err(MaxOverflow::unbounded(MAX_CAP_VAL)),
    both: Err(MAX_OVERFLOW)
});
