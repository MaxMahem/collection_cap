#![allow(dead_code)]

use std::ops::Range;

use collection_cap::Capacity;
use collection_cap::cap::{MaxCapVal, MinCapVal, StaticMaxCap, StaticMinCap, StaticMinMaxCap};
use collection_cap::err::{
    CapError, CapOverflow, CapUnderflow, StaticCapError, StaticCapOverflow, StaticCapUnderflow, StaticFitError,
    StaticFitOverflow, StaticFitUnderflow, UpperBound,
};
use size_hinter::InvalidIterator;

pub const CAP: usize = 10;
pub const OVER_CAP: usize = CAP + 1;
pub const UNDER_CAP: usize = CAP - 1;

pub const COMPAT_ITER: Range<i32> = 0..(CAP as i32);
pub const OVER_ITER: Range<i32> = 0..(OVER_CAP as i32);
pub const UNDER_ITER: Range<i32> = 0..(UNDER_CAP as i32);

pub const MAX_CAP: MaxCapVal = MaxCapVal(CAP);
pub const MIN_CAP: MinCapVal = MinCapVal(CAP);

pub const CAP_OVERFLOWS: CapOverflow = CapOverflow::new(OVER_CAP, MAX_CAP);
pub const CAP_UNDERFLOWS: CapUnderflow = CapUnderflow::new(UNDER_CAP, MIN_CAP);
pub const CAP_ERROR_OVERFLOW: CapError = CapError::Overflow(CAP_OVERFLOWS);
pub const CAP_ERROR_UNDERFLOW: CapError = CapError::Underflow(CAP_UNDERFLOWS);

pub const CAP_RANGE: std::ops::Range<usize> = CAP..CAP + 1;

pub const INVALID_ITER: InvalidIterator<i32> = InvalidIterator::DEFAULT;

pub type MaxCap = StaticMaxCap<CAP>;
pub type MinCap = StaticMinCap<CAP>;
pub type MinMaxCap = StaticMinMaxCap<CAP, CAP>;

pub const STATIC_COMPAT_OVERFLOW: StaticCapOverflow<MinMaxCap> = StaticCapOverflow::<MinMaxCap>::new(OVER_CAP);
pub const STATIC_COMPAT_UNDERFLOW: StaticCapUnderflow<MinMaxCap> = StaticCapUnderflow::<MinMaxCap>::new(UNDER_CAP);

pub const STATIC_COMPAT_ERROR_OVERFLOW: StaticCapError<MinMaxCap> = StaticCapError::Overflow(STATIC_COMPAT_OVERFLOW);
pub const STATIC_COMPAT_ERROR_UNDERFLOW: StaticCapError<MinMaxCap> = StaticCapError::Underflow(STATIC_COMPAT_UNDERFLOW);

pub const STATIC_FIT_OVERFLOW: StaticFitOverflow<MinMaxCap> = StaticFitOverflow::<MinMaxCap>::fixed(OVER_CAP);
pub const STATIC_FIT_UNDERFLOW: StaticFitUnderflow<MinMaxCap> = StaticFitUnderflow::<MinMaxCap>::new(UNDER_CAP);

pub const FIXED_UPPER_BOUND: UpperBound = UpperBound::Fixed(OVER_CAP);

pub const STATIC_FIT_ERROR_OVERFLOW: StaticFitError<MinMaxCap> = StaticFitError::Overflow(STATIC_FIT_OVERFLOW);
pub const STATIC_FIT_ERROR_UNDERFLOW: StaticFitError<MinMaxCap> = StaticFitError::Underflow(STATIC_FIT_UNDERFLOW);
pub const STATIC_FIT_ERROR_BOTH: StaticFitError<MinMaxCap> = StaticFitError::Both {
    overflow: STATIC_FIT_OVERFLOW, //
    underflow: STATIC_FIT_UNDERFLOW,
};
