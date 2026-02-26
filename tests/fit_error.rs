mod common;

use arrayvec::ArrayVec;
use collection_cap::err::{CapError, CapOverflow, CapUnderflow, TargetOverflow, TargetUnderflow};

use common::consts::*;
use common::{check_eq, panics};

const TARGET_OVERFLOW: TargetOverflow<[i32; CAP]> = TargetOverflow::new(CAP + 1);
const TARGET_UNDERFLOW: TargetUnderflow<[i32; CAP]> = TargetUnderflow::new(CAP - 1);

mod cap_error {
    use super::*;

    mod ensure_can_fit {
        use super::*;

        check_eq!(fits: CapError::ensure_can_fit(&FITS_ITER, CAP, CAP) => Ok(()));
        check_eq!(overflow: CapError::ensure_can_fit(&OVER_ITER, CAP, CAP) 
            => Err(CapError::Overflow(CAP_OVERFLOW)));
        check_eq!(underflow: CapError::ensure_can_fit(&UNDER_ITER, CAP, CAP) 
            => Err(CapError::Underflow(CAP_UNDERFLOW)));

        panics!(bad_iter: CapError::ensure_can_fit(&INVALID_ITERATOR, CAP, CAP) 
            => "Invalid size hint: InvalidSizeHint");
    }

    mod from_target_cap {
        use collection_cap::err::TargetCapError;

        use super::*;

        check_eq!(overflow: CapError::from(TARGET_OVERFLOW) => CapError::Overflow(CAP_OVERFLOW));
        check_eq!(underflow: CapError::from(TARGET_UNDERFLOW) => CapError::Underflow(CAP_UNDERFLOW));
        check_eq!(cap_err_overflow: CapError::from(TargetCapError::Overflow(TARGET_OVERFLOW)) => CapError::Overflow(CAP_OVERFLOW));
        check_eq!(cap_err_underflow: CapError::from(TargetCapError::Underflow(TARGET_UNDERFLOW)) => CapError::Underflow(CAP_UNDERFLOW));
    }
}

mod cap_overflow {
    use super::*;

    check_eq!(new: CapOverflow::new(CAP + 1, CAP) => CAP_OVERFLOW);
    panics!(panic_new: CapOverflow::new(CAP, CAP + 1) => "min_size must be greater than max_cap");

    mod ensure_can_fit {
        use super::*;

        check_eq!(fits: CapOverflow::ensure_can_fit(&FITS_ITER, CAP) => Ok(()));
        check_eq!(overflow: CapOverflow::ensure_can_fit(&OVER_ITER, CAP) 
            => Err(CAP_OVERFLOW));

        panics!(bad_iter: CapOverflow::ensure_can_fit(&INVALID_ITERATOR, CAP) 
            => "Invalid size hint: InvalidSizeHint");
    }

    mod ensure_can_fit_into {
        use super::*;

        const CAP_ARRAY_VEC: ArrayVec<i32, CAP> = ArrayVec::new_const();

        check_eq!(fits: CapOverflow::ensure_can_fit_in(&FITS_ITER, &CAP_ARRAY_VEC) => Ok(()));
        check_eq!(overflow: CapOverflow::ensure_can_fit_in(&OVER_ITER, &CAP_ARRAY_VEC) 
            => Err(CAP_OVERFLOW));

        panics!(bad_iter: CapOverflow::ensure_can_fit_in(&INVALID_ITERATOR, &CAP_ARRAY_VEC) 
            => "Invalid size hint: InvalidSizeHint");
    }

    check_eq!(from: CapOverflow::from(TARGET_OVERFLOW) => CAP_OVERFLOW);
}

mod cap_underflow {
    use super::*;

    check_eq!(new: CapUnderflow::new(CAP - 1, CAP) => CAP_UNDERFLOW);
    panics!(panic_new: CapUnderflow::new(CAP, CAP - 1) => "max_size must be less than min_cap");

    mod ensure_can_fit {
        use super::*;

        check_eq!(fits: CapUnderflow::ensure_can_fit(&FITS_ITER, CAP) => Ok(()));
        check_eq!(underflow: CapUnderflow::ensure_can_fit(&UNDER_ITER, CAP) 
            => Err(CAP_UNDERFLOW));

        panics!(bad_iter: CapUnderflow::ensure_can_fit(&INVALID_ITERATOR, CAP) 
            => "Invalid size hint: InvalidSizeHint");
    }

    mod ensure_fits {
        use super::*;

        check_eq!(fits: CapUnderflow::ensure_fits(&FITS_ITER, CAP) => Ok(()));
        check_eq!(underflow: CapUnderflow::ensure_fits(&UNDER_ITER, CAP) 
            => Err(CAP_UNDERFLOW));
    }

    check_eq!(from: CapUnderflow::from(TARGET_UNDERFLOW) => CAP_UNDERFLOW);
}
