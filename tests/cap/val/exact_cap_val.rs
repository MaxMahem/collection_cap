pub use core::ops::Bound;

use crate::common::check_eq;
use crate::common::consts::*;
use crate::{caps, check_compat, check_fit, contains_size, range_bounds};

use collection_cap::cap::{ExactCapVal, MaxCapVal, MinCapVal, MinMaxCapVal, StaticExactCap};
use collection_cap::err::{CompatError, FitError, FitErrorSpan, MaxOverflow, MaxUnderflow, MinOverflow, MinUnderflow};

pub const EXACT_CAP_VAL: ExactCapVal = ExactCapVal(CAP);
const MAX_CAP_VAL: MaxCapVal = MaxCapVal(CAP);
const MIN_CAP_VAL: MinCapVal = MinCapVal(CAP);
const MIN_MAX_CAP_VAL: MinMaxCapVal = MinMaxCapVal::new(CAP, CAP);

check_eq!(capacity: EXACT_CAP_VAL.capacity() => EXACT_CAP_VAL);
check_eq!(zero: ExactCapVal::ZERO => ExactCapVal(0));
check_eq!(from_static: ExactCapVal::from(StaticExactCap::<CAP>) => EXACT_CAP_VAL);

check_eq!(eq: EXACT_CAP_VAL == MIN_MAX_CAP_VAL => true);
check_eq!(ne: EXACT_CAP_VAL != MIN_MAX_CAP_VAL => false);

caps!(EXACT_CAP_VAL => { min: MIN_CAP_VAL, max: MAX_CAP_VAL });

contains_size!(EXACT_CAP_VAL => { cap: true, under: false, over: false });

const MAX_UNDERFLOW: MaxUnderflow<MinCapVal> = MaxUnderflow::<MinCapVal>::new(UNDER_CAP, MIN_CAP_VAL);
const MIN_OVERFLOW: MinOverflow<MaxCapVal> = MinOverflow::<MaxCapVal>::new(OVER_CAP, MAX_CAP_VAL);
check_compat!(EXACT_CAP_VAL => {
    overflow: Err(CompatError::Overflow(MIN_OVERFLOW)),
    underflow: Err(CompatError::Underflow(MAX_UNDERFLOW))
});

const MIN_UNDERFLOW: MinUnderflow<MinCapVal> = MinUnderflow::<MinCapVal>::new(UNDER_CAP, MIN_CAP_VAL);
const MAX_OVERFLOW: MaxOverflow<MaxCapVal> = MaxOverflow::<MaxCapVal>::fixed(OVER_CAP, MAX_CAP_VAL);
const MAX_OVERFLOW_UNBOUNDED: MaxOverflow<MaxCapVal> = MaxOverflow::<MaxCapVal>::unbounded(MAX_CAP_VAL);
const FIT_ERROR_SPAN: FitErrorSpan<MinCapVal, MaxCapVal> = FitErrorSpan::new(MAX_OVERFLOW, MIN_UNDERFLOW);
check_fit!(EXACT_CAP_VAL => {
    underflow: Err(FitError::Underflow(MIN_UNDERFLOW)),
    overflow: Err(FitError::Overflow(MAX_OVERFLOW)),
    unbounded: Err(FitError::Overflow(MAX_OVERFLOW_UNBOUNDED)),
    both: Err(FitError::Both(FIT_ERROR_SPAN))
});

range_bounds!(EXACT_CAP_VAL => { start: Bound::Included(&CAP), end: Bound::Included(&CAP) });
