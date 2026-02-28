use collection_cap::err::{CapOverflow, CapUnderflow, Overflows, Underflows, VarCapError};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

const TARGET_OVERFLOW: CapOverflow<[i32; CAP]> = CapOverflow::new(OVER_CAP);
const TARGET_UNDERFLOW: CapUnderflow<[i32; CAP]> = CapUnderflow::new(UNDER_CAP);

mod val_cap_error {
    use super::*;

    mod from_cap {
        use collection_cap::err::CapError;

        use super::*;

        check_eq!(overflow: VarCapError::from(TARGET_OVERFLOW) => VarCapError::Overflows(CAP_OVERFLOWS));
        check_eq!(underflow: VarCapError::from(TARGET_UNDERFLOW) => VarCapError::Underflows(CAP_UNDERFLOWS));
        check_eq!(cap_err_overflow: VarCapError::from(CapError::Overflow(TARGET_OVERFLOW)) => VarCapError::Overflows(CAP_OVERFLOWS));
        check_eq!(cap_err_underflow: VarCapError::from(CapError::Underflow(TARGET_UNDERFLOW)) => VarCapError::Underflows(CAP_UNDERFLOWS));
    }
}

mod overflows {
    use super::*;

    check_eq!(new: Overflows::new(OVER_CAP, MAX_CAP) => CAP_OVERFLOWS);
    panics!(panic_new: Overflows::new(CAP, MAX_CAP) => "min_size must be > max_cap");
    check_eq!(min_size: CAP_OVERFLOWS.min_size() => OVER_CAP);
    check_eq!(max_cap: CAP_OVERFLOWS.max_cap() => MAX_CAP);

    check_eq!(from: Overflows::from(TARGET_OVERFLOW) => CAP_OVERFLOWS);
}

mod underflows {
    use super::*;

    check_eq!(new: Underflows::new(UNDER_CAP, MIN_CAP) => CAP_UNDERFLOWS);
    panics!(panic_new: Underflows::new(CAP, MIN_CAP) => "max_size must be < min_cap");
    check_eq!(max_size: CAP_UNDERFLOWS.max_size() => UNDER_CAP);
    check_eq!(min_cap: CAP_UNDERFLOWS.min_cap() => MIN_CAP);

    check_eq!(from: Underflows::from(TARGET_UNDERFLOW) => CAP_UNDERFLOWS);
}
