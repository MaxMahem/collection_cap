#![allow(dead_code)]

use std::ops::{Range, RangeFrom, RangeInclusive};

use collection_cap::cap::{
    ExactCapVal, MaxCapVal, MinCapVal, MinMaxCapVal, StaticMaxCap, StaticMinCap, StaticMinMaxCap,
};
use collection_cap::err::{
    CompatError, FitError, FitErrorSpan, MaxOverflow, MaxUnderflow, MinOverflow, MinUnderflow, UpperBound,
};
use size_hinter::{InvalidIterator, TestIterator};

pub mod base {
    use super::*;
    pub const CAP: usize = 10;
    pub const OVER_CAP: usize = CAP + 1;
    pub const UNDER_CAP: usize = CAP - 1;

    pub const CAP_RANGE: RangeInclusive<usize> = CAP..=CAP;
    pub const FIXED_UPPER_BOUND: UpperBound = UpperBound::Fixed(OVER_CAP);
}

pub mod iter {
    use super::*;
    use super::base::*;
    pub const COMPAT_ITER: Range<i32> = 0..(CAP as i32);
    pub const OVER_ITER: Range<i32> = 0..(OVER_CAP as i32);
    pub const OVER_ITER_UNBOUNDED: RangeFrom<i32> = 0..;
    pub const UNDER_ITER: Range<i32> = 0..(UNDER_CAP as i32);

    pub const BOTH_ITER: TestIterator<i32> = TestIterator::new((UNDER_CAP, Some(OVER_CAP)));
    pub const INVALID_ITER: InvalidIterator<i32> = InvalidIterator::DEFAULT;
}

pub mod val {
    use super::*;
    use super::base::*;
    pub const MAX_CAP_VAL: MaxCapVal = MaxCapVal(CAP);
    pub const MIN_CAP_VAL: MinCapVal = MinCapVal(CAP);
    pub const MIN_MAX_CAP_VAL: MinMaxCapVal = MinMaxCapVal::new(CAP, CAP);
    pub const EXACT_CAP_VAL: ExactCapVal = ExactCapVal(CAP);
}

pub mod stat {
    use super::*;
    use super::base::*;
    pub type MinMaxCap = StaticMinMaxCap<CAP, CAP>;
    pub type MinCap = StaticMinCap<CAP>;
    pub type MaxCap = StaticMaxCap<CAP>;
}

pub mod err_val_compat {
    use super::*;
    use super::base::*;
    use super::val::*;
    pub const MIN_OVERFLOWS: MinOverflow<MaxCapVal> = MinOverflow::<MaxCapVal>::new(OVER_CAP, MAX_CAP_VAL);
    pub const MAX_UNDERFLOWS: MaxUnderflow<MinCapVal> = MaxUnderflow::<MinCapVal>::new(UNDER_CAP, MIN_CAP_VAL);
    pub const OVERFLOW: CompatError<MinCapVal, MaxCapVal> = CompatError::Overflow(MIN_OVERFLOWS);
    pub const UNDERFLOW: CompatError<MinCapVal, MaxCapVal> = CompatError::Underflow(MAX_UNDERFLOWS);
}

pub mod err_val_fit {
    use super::*;
    use super::base::*;
    use super::val::*;
    pub const MIN_UNDERFLOWS: MinUnderflow<MinCapVal> = MinUnderflow::<MinCapVal>::new(UNDER_CAP, MIN_CAP_VAL);
    pub const MAX_OVERFLOWS: MaxOverflow<MaxCapVal> = MaxOverflow::<MaxCapVal>::fixed(OVER_CAP, MAX_CAP_VAL);
    pub const MAX_OVERFLOWS_UNBOUNDED: MaxOverflow<MaxCapVal> = MaxOverflow::<MaxCapVal>::unbounded(MAX_CAP_VAL);
    pub const SPAN: FitErrorSpan<MinCapVal, MaxCapVal> = FitErrorSpan::new(MAX_OVERFLOWS, MIN_UNDERFLOWS);

    pub const OVERFLOW: FitError<MinCapVal, MaxCapVal> = FitError::Overflow(MAX_OVERFLOWS);
    pub const UNDERFLOW: FitError<MinCapVal, MaxCapVal> = FitError::Underflow(MIN_UNDERFLOWS);
    pub const BOTH: FitError<MinCapVal, MaxCapVal> = FitError::Both(SPAN);
}

pub mod err_stat_compat {
    use super::*;
    use super::base::*;
    use super::stat::*;
    pub const MIN_OVERFLOWS: MinOverflow<MaxCap> = MinOverflow::<MaxCap>::new(OVER_CAP);
    pub const MAX_UNDERFLOWS: MaxUnderflow<MinCap> = MaxUnderflow::<MinCap>::new(UNDER_CAP);
    pub const OVERFLOW: CompatError<MinCap, MaxCap> = CompatError::Overflow(MIN_OVERFLOWS);
    pub const UNDERFLOW: CompatError<MinCap, MaxCap> = CompatError::Underflow(MAX_UNDERFLOWS);
}

pub mod err_stat_fit {
    use super::*;
    use super::base::*;
    use super::stat::*;
    pub const MIN_UNDERFLOWS: MinUnderflow<MinCap> = MinUnderflow::<MinCap>::new(UNDER_CAP);
    pub const MAX_OVERFLOWS: MaxOverflow<MaxCap> = MaxOverflow::<MaxCap>::fixed(OVER_CAP);
    pub const MAX_OVERFLOWS_UNBOUNDED: MaxOverflow<MaxCap> = MaxOverflow::<MaxCap>::UNBOUNDED;
    pub const SPAN: FitErrorSpan<MinCap, MaxCap> = FitErrorSpan::new(MAX_OVERFLOWS, MIN_UNDERFLOWS);

    pub const OVERFLOW: FitError<MinCap, MaxCap> = FitError::Overflow(MAX_OVERFLOWS);
    pub const OVERFLOW_UNBOUNDED: FitError<MinCap, MaxCap> = FitError::Overflow(MAX_OVERFLOWS_UNBOUNDED);
    pub const UNDERFLOW: FitError<MinCap, MaxCap> = FitError::Underflow(MIN_UNDERFLOWS);
    pub const BOTH: FitError<MinCap, MaxCap> = FitError::Both(SPAN);
}
