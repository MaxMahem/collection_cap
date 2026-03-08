use collection_cap::cap::{MaxCapVal, MinCapVal};
use collection_cap::err::{OverlapError, OverlapErrorSpan, MaxOverflow, MinUnderflow, UpperBound};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

const MAX_CAP_VAL: MaxCapVal = MaxCapVal(CAP);
const MIN_CAP_VAL: MinCapVal = MinCapVal(CAP);

pub const MIN_UNDERFLOW: MinUnderflow<MinCapVal> = MinUnderflow::<MinCapVal>::new(UNDER_CAP, MIN_CAP_VAL);
pub const MAX_OVERFLOW: MaxOverflow<MaxCapVal> = MaxOverflow::<MaxCapVal>::fixed(OVER_CAP, MAX_CAP_VAL);
pub const MAX_OVERFLOW_UNBOUNDED: MaxOverflow<MaxCapVal> = MaxOverflow::<MaxCapVal>::unbounded(MAX_CAP_VAL);

pub const OVERLAP_ERR_OVERFLOW: OverlapError<MinCapVal, MaxCapVal> = OverlapError::Overflow(MAX_OVERFLOW);
pub const OVERLAP_ERR_UNDERFLOW: OverlapError<MinCapVal, MaxCapVal> = OverlapError::Underflow(MIN_UNDERFLOW);
pub const OVERLAP_ERR_BOTH: OverlapError<MinCapVal, MaxCapVal> = OverlapError::Both(OverlapErrorSpan::new(MAX_OVERFLOW, MIN_UNDERFLOW));

const FIXED_UPPER_BOUND: UpperBound = UpperBound::Fixed(OVER_CAP);
const SPAN: OverlapErrorSpan<MinCapVal, MaxCapVal> = OverlapErrorSpan::new(MAX_OVERFLOW, MIN_UNDERFLOW);

pub const CONST_MIN_UNDERFLOW: MinUnderflow<ConstMinCap<CAP>> = MinUnderflow::<ConstMinCap<CAP>>::new(UNDER_CAP);
pub const CONST_MAX_OVERFLOW: MaxOverflow<ConstMaxCap<CAP>> = MaxOverflow::<ConstMaxCap<CAP>>::fixed(OVER_CAP);
pub const CONST_OVERLAP_ERR_OVERFLOW: OverlapError<ConstMinCap<CAP>, ConstMaxCap<CAP>> = OverlapError::Overflow(CONST_MAX_OVERFLOW);
pub const CONST_OVERLAP_ERR_UNDERFLOW: OverlapError<ConstMinCap<CAP>, ConstMaxCap<CAP>> =
    OverlapError::Underflow(CONST_MIN_UNDERFLOW);
pub const CONST_FIT_ERR_SPAN: OverlapErrorSpan<ConstMinCap<CAP>, ConstMaxCap<CAP>> =
    OverlapErrorSpan::new(CONST_MAX_OVERFLOW, CONST_MIN_UNDERFLOW);
pub const CONST_OVERLAP_ERR_BOTH: OverlapError<ConstMinCap<CAP>, ConstMaxCap<CAP>> = OverlapError::Both(CONST_FIT_ERR_SPAN);

mod underflows {
    use super::*;

    mod dynamic {
        use super::*;

        check_eq!(new: MinUnderflow::<MinCapVal>::new(UNDER_CAP, MIN_CAP_VAL) => MIN_UNDERFLOW);
        panics!(panic_new: MinUnderflow::<MinCapVal>::new(CAP, MIN_CAP_VAL) => "min_size must be < min_cap");
        check_eq!(min_size: MIN_UNDERFLOW.min_size() => UNDER_CAP);
        check_eq!(min_cap: MIN_UNDERFLOW.min_cap() => &MIN_CAP_VAL);
    }

    mod const_cap {
        use super::*;

        check_eq!(new: MinUnderflow::<ConstMinCap<CAP>>::new(UNDER_CAP) => CONST_MIN_UNDERFLOW);
        panics!(panic_new: MinUnderflow::<ConstMinCap<CAP>>::new(CAP) => "min_size must be < MIN");
        check_eq!(min_size: CONST_MIN_UNDERFLOW.min_size() => UNDER_CAP);
        check_eq!(min_cap: *CONST_MIN_UNDERFLOW.min_cap() => ConstMinCap::<CAP>::CAP);
    }
}

mod overflows {
    use super::*;

    mod variable_cap {
        use super::*;

        check_eq!(fixed: MaxOverflow::<MaxCapVal>::fixed(OVER_CAP, MAX_CAP_VAL)
            => MAX_OVERFLOW);
        panics!(panic_fixed: MaxOverflow::<MaxCapVal>::fixed(CAP, MAX_CAP_VAL)
            => "max_size must be > max_cap");
        check_eq!(unbounded: MaxOverflow::unbounded(MAX_CAP_VAL)
            => MAX_OVERFLOW_UNBOUNDED);
        check_eq!(max_size: MAX_OVERFLOW.max_size() => FIXED_UPPER_BOUND);
        check_eq!(max_cap: *MAX_OVERFLOW.max_cap() => MAX_CAP_VAL);
    }

    mod const_cap {
        use super::*;

        check_eq!(fixed: MaxOverflow::<ConstMaxCap<CAP>>::fixed(OVER_CAP) => CONST_MAX_OVERFLOW);
        panics!(panic_fixed: MaxOverflow::<ConstMaxCap<CAP>>::fixed(CAP) => "max_size must be > MAX");
        check_eq!(unbounded: MaxOverflow::<ConstMaxCap<CAP>>::UNBOUNDED.max_size() 
            => UpperBound::Unbounded);
        check_eq!(fixed_max_size: CONST_MAX_OVERFLOW.max_size() => FIXED_UPPER_BOUND);
        check_eq!(max_cap: *CONST_MAX_OVERFLOW.max_cap() => ConstMaxCap::<CAP>::CAP);
    }
}

mod overlap_error_span {
    use super::*;

    const OVERLAP_ERROR_SPAN: OverlapErrorSpan<MinCapVal, MaxCapVal> = OverlapErrorSpan::new(MAX_OVERFLOW, MIN_UNDERFLOW);

    check_eq!(new: OverlapErrorSpan::new(MAX_OVERFLOW, MIN_UNDERFLOW) => OVERLAP_ERROR_SPAN);
    check_eq!(overflow: OVERLAP_ERROR_SPAN.overflow() => &MAX_OVERFLOW);
    check_eq!(underflow: OVERLAP_ERROR_SPAN.underflow() => &MIN_UNDERFLOW);

    panics!(panic_new: OverlapErrorSpan::new(
        MaxOverflow::<MaxCapVal>::fixed(10, MaxCapVal(5)),
        MinUnderflow::<MinCapVal>::new(20, MinCapVal(25))
    ) => "underflow and overflow must intersect");
}

mod errors {
    use super::*;

    mod dynamic {
        use super::*;

        check_eq!(from_overflow: OverlapError::from(MAX_OVERFLOW) => OVERLAP_ERR_OVERFLOW);
        check_eq!(from_underflow: OverlapError::from(MIN_UNDERFLOW) => OVERLAP_ERR_UNDERFLOW);
        check_eq!(from_both: OverlapError::from(SPAN) => OVERLAP_ERR_BOTH);
    }

    mod const_cap {
        use super::*;

        check_eq!(from_overflow: OverlapError::from(CONST_MAX_OVERFLOW) => CONST_OVERLAP_ERR_OVERFLOW);
        check_eq!(from_underflow: OverlapError::from(CONST_MIN_UNDERFLOW) => CONST_OVERLAP_ERR_UNDERFLOW);
        check_eq!(from_both: OverlapError::from(CONST_FIT_ERR_SPAN) => CONST_OVERLAP_ERR_BOTH);
    }
}
