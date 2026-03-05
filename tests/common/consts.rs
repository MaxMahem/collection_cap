#![allow(dead_code)]

use std::ops::{Range, RangeFrom, RangeInclusive};

use collection_cap::cap::{
    ExactCapVal, MaxCapVal, MinCapVal, MinMaxCapVal, StaticMaxCap, StaticMinCap, StaticMinMaxCap,
};
use collection_cap::err::{
    CompatError, FitError, FitErrorSpan, MaxOverflow, MaxUnderflow, MinOverflow, MinUnderflow, UpperBound,
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
pub const EXACT_CAP_VAL: ExactCapVal = ExactCapVal(CAP);

pub const MIN_OVERFLOWS: MinOverflow<MaxCapVal> = MinOverflow::<MaxCapVal>::new(OVER_CAP, MAX_CAP_VAL);
pub const MAX_UNDERFLOWS: MaxUnderflow<MinCapVal> = MaxUnderflow::<MinCapVal>::new(UNDER_CAP, MIN_CAP_VAL);
pub const CAP_ERROR_OVERFLOW: CompatError<MinCapVal, MaxCapVal> = CompatError::Overflow(MIN_OVERFLOWS);
pub const CAP_ERROR_UNDERFLOW: CompatError<MinCapVal, MaxCapVal> = CompatError::Underflow(MAX_UNDERFLOWS);

pub const CAP_RANGE: RangeInclusive<usize> = CAP..=CAP;

pub const INVALID_ITER: InvalidIterator<i32> = InvalidIterator::DEFAULT;

pub type MinMaxCap = StaticMinMaxCap<CAP, CAP>;
pub type MinCap = StaticMinCap<CAP>;
pub type MaxCap = StaticMaxCap<CAP>;

pub const FIXED_UPPER_BOUND: UpperBound = UpperBound::Fixed(OVER_CAP);
pub const BOTH_ITER: TestIterator<i32> = TestIterator::new((UNDER_CAP, Some(OVER_CAP)));

// --- FIT ---

pub const MIN_UNDERFLOWS: MinUnderflow<MinCapVal> = MinUnderflow::<MinCapVal>::new(UNDER_CAP, MIN_CAP_VAL);
pub const MAX_OVERFLOWS: MaxOverflow<MaxCapVal> = MaxOverflow::<MaxCapVal>::fixed(OVER_CAP, MAX_CAP_VAL);
pub const MAX_OVERFLOWS_UNBOUNDED: MaxOverflow<MaxCapVal> = MaxOverflow::<MaxCapVal>::unbounded(MAX_CAP_VAL);
pub const FIT_ERROR_SPAN: FitErrorSpan<MinCapVal, MaxCapVal> = FitErrorSpan::new(MAX_OVERFLOWS, MIN_UNDERFLOWS);

pub const FIT_ERROR_OVERFLOW: FitError<MinCapVal, MaxCapVal> = FitError::Overflow(MAX_OVERFLOWS);
pub const FIT_ERROR_UNDERFLOW: FitError<MinCapVal, MaxCapVal> = FitError::Underflow(MIN_UNDERFLOWS);
pub const FIT_ERROR_BOTH: FitError<MinCapVal, MaxCapVal> = FitError::Both(FIT_ERROR_SPAN);
