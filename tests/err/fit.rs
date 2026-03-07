use collection_cap::StaticCap;
use collection_cap::cap::{MaxCapVal, MinCapVal};
use collection_cap::err::{FitError, FitErrorSpan, MaxOverflow, MinUnderflow, UpperBound};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

const MAX_CAP_VAL: MaxCapVal = MaxCapVal(CAP);
const MIN_CAP_VAL: MinCapVal = MinCapVal(CAP);

pub const MIN_UNDERFLOW: MinUnderflow<MinCapVal> = MinUnderflow::<MinCapVal>::new(UNDER_CAP, MIN_CAP_VAL);
pub const MAX_OVERFLOW: MaxOverflow<MaxCapVal> = MaxOverflow::<MaxCapVal>::fixed(OVER_CAP, MAX_CAP_VAL);
pub const MAX_OVERFLOW_UNBOUNDED: MaxOverflow<MaxCapVal> = MaxOverflow::<MaxCapVal>::unbounded(MAX_CAP_VAL);

pub const FIT_ERR_OVERFLOW: FitError<MinCapVal, MaxCapVal> = FitError::Overflow(MAX_OVERFLOW);
pub const FIT_ERR_UNDERFLOW: FitError<MinCapVal, MaxCapVal> = FitError::Underflow(MIN_UNDERFLOW);
pub const FIT_ERR_BOTH: FitError<MinCapVal, MaxCapVal> = FitError::Both(FitErrorSpan::new(MAX_OVERFLOW, MIN_UNDERFLOW));

const FIXED_UPPER_BOUND: UpperBound = UpperBound::Fixed(OVER_CAP);
const SPAN: FitErrorSpan<MinCapVal, MaxCapVal> = FitErrorSpan::new(MAX_OVERFLOW, MIN_UNDERFLOW);

pub const STATIC_MIN_UNDERFLOW: MinUnderflow<StaticMinCap<CAP>> = MinUnderflow::<StaticMinCap<CAP>>::new(UNDER_CAP);
pub const STATIC_MAX_OVERFLOW: MaxOverflow<StaticMaxCap<CAP>> = MaxOverflow::<StaticMaxCap<CAP>>::fixed(OVER_CAP);
pub const STATIC_FIT_ERR_OVERFLOW: FitError<StaticMinCap<CAP>, StaticMaxCap<CAP>> =
    FitError::Overflow(STATIC_MAX_OVERFLOW);
pub const STATIC_FIT_ERR_UNDERFLOW: FitError<StaticMinCap<CAP>, StaticMaxCap<CAP>> =
    FitError::Underflow(STATIC_MIN_UNDERFLOW);
pub const STATIC_FIT_ERR_SPAN: FitErrorSpan<StaticMinCap<CAP>, StaticMaxCap<CAP>> =
    FitErrorSpan::new(STATIC_MAX_OVERFLOW, STATIC_MIN_UNDERFLOW);
pub const STATIC_FIT_ERR_BOTH: FitError<StaticMinCap<CAP>, StaticMaxCap<CAP>> = FitError::Both(STATIC_FIT_ERR_SPAN);

mod underflows {
    use super::*;

    mod dynamic {
        use super::*;

        check_eq!(new: MinUnderflow::<MinCapVal>::new(UNDER_CAP, MIN_CAP_VAL) => MIN_UNDERFLOW);
        panics!(panic_new: MinUnderflow::<MinCapVal>::new(CAP, MIN_CAP_VAL) => "min_size must be < min_cap");
        check_eq!(min_size: MIN_UNDERFLOW.min_size() => UNDER_CAP);
        check_eq!(min_cap: MIN_UNDERFLOW.min_cap() => &MIN_CAP_VAL);
    }

    mod static_cap {
        use super::*;

        check_eq!(new: MinUnderflow::<StaticMinCap<CAP>>::new(UNDER_CAP) => STATIC_MIN_UNDERFLOW);
        panics!(panic_new: MinUnderflow::<StaticMinCap<CAP>>::new(CAP) => "min_size must be < MIN");
        check_eq!(min_size: STATIC_MIN_UNDERFLOW.min_size() => UNDER_CAP);
        check_eq!(min_cap: *STATIC_MIN_UNDERFLOW.min_cap() => StaticMinCap::<CAP>::CAP);
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

    mod static_cap {
        use super::*;

        check_eq!(fixed: MaxOverflow::<StaticMaxCap<CAP>>::fixed(OVER_CAP) => STATIC_MAX_OVERFLOW);
        panics!(panic_fixed: MaxOverflow::<StaticMaxCap<CAP>>::fixed(CAP) => "max_size must be > MAX");
        check_eq!(unbounded: MaxOverflow::<StaticMaxCap<CAP>>::UNBOUNDED.max_size() 
            => UpperBound::Unbounded);
        check_eq!(fixed_max_size: STATIC_MAX_OVERFLOW.max_size() => FIXED_UPPER_BOUND);
        check_eq!(max_cap: *STATIC_MAX_OVERFLOW.max_cap() => StaticMaxCap::<CAP>::CAP);
    }
}

mod fit_error_span {
    use super::*;

    const FIT_ERROR_SPAN: FitErrorSpan<MinCapVal, MaxCapVal> = FitErrorSpan::new(MAX_OVERFLOW, MIN_UNDERFLOW);

    check_eq!(new: FitErrorSpan::new(MAX_OVERFLOW, MIN_UNDERFLOW) => FIT_ERROR_SPAN);
    check_eq!(overflow: FIT_ERROR_SPAN.overflow() => &MAX_OVERFLOW);
    check_eq!(underflow: FIT_ERROR_SPAN.underflow() => &MIN_UNDERFLOW);

    panics!(panic_new: FitErrorSpan::new(
        MaxOverflow::<MaxCapVal>::fixed(10, MaxCapVal(5)),
        MinUnderflow::<MinCapVal>::new(20, MinCapVal(25))
    ) => "underflow and overflow must intersect");
}

mod errors {
    use super::*;

    mod dynamic {
        use super::*;

        check_eq!(from_overflow: FitError::from(MAX_OVERFLOW) => FIT_ERR_OVERFLOW);
        check_eq!(from_underflow: FitError::from(MIN_UNDERFLOW) => FIT_ERR_UNDERFLOW);
        check_eq!(from_both: FitError::from(SPAN) => FIT_ERR_BOTH);
    }

    mod static_cap {
        use super::*;

        check_eq!(from_overflow: FitError::from(STATIC_MAX_OVERFLOW) => STATIC_FIT_ERR_OVERFLOW);
        check_eq!(from_underflow: FitError::from(STATIC_MIN_UNDERFLOW) => STATIC_FIT_ERR_UNDERFLOW);
        check_eq!(from_both: FitError::from(STATIC_FIT_ERR_SPAN) => STATIC_FIT_ERR_BOTH);
    }
}
