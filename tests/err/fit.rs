use collection_cap::err::{FitOverflow, FitUnderflow, StaticFitError, UpperBound};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

mod static_fit_error {
    use super::*;

    check_eq!(from_overflow: StaticFitError::<MinMaxCap>::from(STATIC_FIT_OVERFLOW) 
        => StaticFitError::Overflow(STATIC_FIT_OVERFLOW));
    check_eq!(from_underflow: StaticFitError::<MinMaxCap>::from(STATIC_FIT_UNDERFLOW) 
        => StaticFitError::Underflow(STATIC_FIT_UNDERFLOW));
}

mod overflows {

    use super::*;

    check_eq!(fixed: FitOverflow::<MinMaxCap>::fixed_static(OVER_CAP) => STATIC_FIT_OVERFLOW);
    check_eq!(unbounded: FitOverflow::<MinMaxCap>::UNBOUNDED.max_size() => UpperBound::Unbounded);
    check_eq!(fixed_max_size: STATIC_FIT_OVERFLOW.max_size() => FIXED_UPPER_BOUND);
    panics!(panic_bounded: FitOverflow::<MinMaxCap>::fixed_static(CAP) => "max_size must be > C::MAX_CAP");
}

mod underflows {
    use super::*;

    check_eq!(new: FitUnderflow::<MinMaxCap>::new_static(UNDER_CAP) => STATIC_FIT_UNDERFLOW);
    check_eq!(min_size: STATIC_FIT_UNDERFLOW.min_size() => UNDER_CAP);
    panics!(panic_new: FitUnderflow::<MinMaxCap>::new_static(CAP) => "min_size must be < C::MIN_CAP");
}
