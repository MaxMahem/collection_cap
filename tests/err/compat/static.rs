use collection_cap::err::{CapError, CapOverflow, CapUnderflow, StaticCapError, StaticCapOverflow, StaticCapUnderflow};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

mod static_cap_error {
    use super::*;

    check_eq!(from_overflow: StaticCapError::<MinMaxCap>::from(STATIC_COMPAT_OVERFLOW) => STATIC_COMPAT_ERROR_OVERFLOW);
    check_eq!(from_underflow: StaticCapError::<MinMaxCap>::from(STATIC_COMPAT_UNDERFLOW) => STATIC_COMPAT_ERROR_UNDERFLOW);

    check_eq!(to_cap_error_overflow: CapError::from(STATIC_COMPAT_ERROR_OVERFLOW) => CAP_ERROR_OVERFLOW);
    check_eq!(to_cap_error_underflow: CapError::from(STATIC_COMPAT_ERROR_UNDERFLOW) => CAP_ERROR_UNDERFLOW);
}

mod overflows {
    use super::*;

    const STATIC_OVERFLOWS: StaticCapOverflow<MaxCap> = StaticCapOverflow::<MaxCap>::new(OVER_CAP);

    check_eq!(new: StaticCapOverflow::<MaxCap>::new(OVER_CAP) => STATIC_OVERFLOWS);
    panics!(panic_new: StaticCapOverflow::<MaxCap>::new(CAP) => "min_size must be > C::MAX_CAP");
    check_eq!(min_size: STATIC_OVERFLOWS.min_size() => OVER_CAP);
    check_eq!(to_cap_overflow: CapOverflow::from(STATIC_OVERFLOWS) => CAP_OVERFLOWS);
}

mod underflows {
    use super::*;

    const STATIC_UNDERFLOWS: StaticCapUnderflow<MinCap> = StaticCapUnderflow::<MinCap>::new(UNDER_CAP);

    check_eq!(new: StaticCapUnderflow::<MinCap>::new(UNDER_CAP) => STATIC_UNDERFLOWS);
    panics!(panic_new: StaticCapUnderflow::<MinCap>::new(CAP) => "max_size must be < C::MIN_CAP");
    check_eq!(max_size: STATIC_UNDERFLOWS.max_size() => UNDER_CAP);
    check_eq!(to_cap_underflow: CapUnderflow::from(STATIC_UNDERFLOWS) => CAP_UNDERFLOWS);
}
