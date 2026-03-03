use collection_cap::err::{FitError, FitOverflow, FitUnderflow, StaticFitError, UpperBound};

use crate::common::check_eq;
use crate::common::consts::*;

mod static_fit_error {
    use super::*;

    check_eq!(from_overflow: StaticFitError::<MinMaxCap>::from(FitOverflow::<MaxCap>::fixed(OVER_CAP)) 
        => FitError::Overflow(FitOverflow::<MaxCap>::fixed(OVER_CAP)));
    check_eq!(from_underflow: StaticFitError::<MinMaxCap>::from(FitUnderflow::<MinCap>::new(UNDER_CAP)) 
        => FitError::Underflow(FitUnderflow::<MinCap>::new(UNDER_CAP)));
}

mod overflows {

    use super::*;

    check_eq!(fixed: FitOverflow::<MaxCap>::fixed(OVER_CAP) => FitOverflow::<MaxCap>::fixed(OVER_CAP));
    check_eq!(unbounded: FitOverflow::<MaxCap>::UNBOUNDED.max_size() => UpperBound::Unbounded);
    check_eq!(fixed_max_size: FitOverflow::<MaxCap>::fixed(OVER_CAP).max_size() => FIXED_UPPER_BOUND);
}

mod underflows {
    use super::*;

    check_eq!(new: FitUnderflow::<MinCap>::new(UNDER_CAP) => FitUnderflow::<MinCap>::new(UNDER_CAP));
    check_eq!(min_size: FitUnderflow::<MinCap>::new(UNDER_CAP).min_size() => UNDER_CAP);
}
