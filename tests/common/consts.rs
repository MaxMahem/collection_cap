#![allow(dead_code)]

use std::ops::{Range, RangeFrom};

use collection_cap::Capacity;
use collection_cap::cap::{
    MaxCapVal, MinCapVal, MinMaxCapVal, StaticExactCap, StaticMaxCap, StaticMinCap, StaticMinMaxCap,
};
use collection_cap::err::{
    CompatError, FitBoth, FitError, FitOverflow, FitUnderflow, MaxUnderflow, MinOverflow, StaticCapError,
    StaticFitError, UpperBound, VarCapError,
};
use size_hinter::{InvalidIterator, TestIterator};

pub const CAP: usize = 10;
pub const OVER_CAP: usize = CAP + 1;
pub const UNDER_CAP: usize = CAP - 1;

pub const COMPAT_ITER: Range<i32> = 0..(CAP as i32);
pub const OVER_ITER: Range<i32> = 0..(OVER_CAP as i32);
pub const OVER_ITER_UNBOUNDED: RangeFrom<i32> = 0..;
pub const UNDER_ITER: Range<i32> = 0..(UNDER_CAP as i32);

pub const MAX_CAP_VAL: MaxCapVal = MaxCapVal(CAP);
pub const MIN_CAP_VAL: MinCapVal = MinCapVal(CAP);
pub const MIN_MAX_CAP_VAL: MinMaxCapVal = MinMaxCapVal::new(CAP, CAP);

pub const MIN_OVERFLOWS: MinOverflow<MaxCapVal> = MinOverflow::from_parts_unchecked(OVER_CAP, MAX_CAP_VAL);
pub const MAX_UNDERFLOWS: MaxUnderflow<MinCapVal> = MaxUnderflow::from_parts_unchecked(UNDER_CAP, MIN_CAP_VAL);
pub const CAP_ERROR_OVERFLOW: VarCapError = CompatError::Overflow(MIN_OVERFLOWS);
pub const CAP_ERROR_UNDERFLOW: VarCapError = CompatError::Underflow(MAX_UNDERFLOWS);

pub const CAP_RANGE: Range<usize> = CAP..CAP + 1;

pub const INVALID_ITER: InvalidIterator<i32> = InvalidIterator::DEFAULT;

pub type MinMaxCap = StaticMinMaxCap<CAP, CAP>;

pub const STATIC_MIN_OVERFLOW: MinOverflow<MinMaxCap> = MinOverflow::<MinMaxCap>::new_static(OVER_CAP);
pub const STATIC_MAX_UNDERFLOW: MaxUnderflow<MinMaxCap> = MaxUnderflow::<MinMaxCap>::new_static(UNDER_CAP);

pub const STATIC_CAP_ERROR_OVERFLOW: StaticCapError<MinMaxCap> =
    StaticCapError::Overflow(MinOverflow::<MinMaxCap>::new_static(OVER_CAP));
pub const STATIC_CAP_ERROR_UNDERFLOW: StaticCapError<MinMaxCap> =
    StaticCapError::Underflow(MaxUnderflow::<MinMaxCap>::new_static(UNDER_CAP));

// --- FIT ---

pub const STATIC_FIT_OVERFLOW: FitOverflow<MinMaxCap> = FitOverflow::<MinMaxCap>::fixed_static(OVER_CAP);
pub const STATIC_FIT_UNDERFLOW: FitUnderflow<MinMaxCap> = FitUnderflow::<MinMaxCap>::new_static(UNDER_CAP);

pub const FIXED_UPPER_BOUND: UpperBound = UpperBound::Fixed(OVER_CAP);

pub const STATIC_FIT_ERROR_OVERFLOW: StaticFitError<MinMaxCap> = StaticFitError::Overflow(STATIC_FIT_OVERFLOW);
pub const STATIC_FIT_ERROR_UNDERFLOW: StaticFitError<MinMaxCap> = StaticFitError::Underflow(STATIC_FIT_UNDERFLOW);
pub const STATIC_FIT_ERROR_BOTH: StaticFitError<MinMaxCap> = StaticFitError::Both(STATIC_FIT_BOTH);

pub const BOTH_ITER: TestIterator<i32> = TestIterator::new((UNDER_CAP, Some(OVER_CAP)));

pub const STATIC_FIT_BOTH: FitBoth<MinMaxCap, MinMaxCap> =
    FitBoth::from_parts_unchecked(STATIC_FIT_OVERFLOW, STATIC_FIT_UNDERFLOW);

// Variable version of fit errors
pub const VAR_FIT_OVERFLOW: FitOverflow<MaxCapVal> = FitOverflow::from_parts_unchecked(OVER_CAP, MAX_CAP_VAL);
pub const VAR_FIT_UNDERFLOW: FitUnderflow<MinCapVal> = FitUnderflow::from_parts_unchecked(UNDER_CAP, MIN_CAP_VAL);

pub const VAR_FIT_ERROR_OVERFLOW: FitError<MinCapVal, MaxCapVal> = FitError::Overflow(VAR_FIT_OVERFLOW);
pub const VAR_FIT_ERROR_UNDERFLOW: FitError<MinCapVal, MaxCapVal> = FitError::Underflow(VAR_FIT_UNDERFLOW);

pub const VAR_FIT_BOTH: FitBoth<MinCapVal, MaxCapVal> =
    FitBoth::from_parts_unchecked(VAR_FIT_OVERFLOW, VAR_FIT_UNDERFLOW);
pub const VAR_FIT_ERROR_BOTH: FitError<MinCapVal, MaxCapVal> = FitError::Both(VAR_FIT_BOTH);
