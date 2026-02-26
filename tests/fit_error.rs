mod common;

use arrayvec::ArrayVec;
use collection_cap::err::{FitError, Overflows, Underflows, CapOverflow, CapUnderflow};

use common::consts::*;
use common::{check_eq, panics};

const TARGET_OVERFLOW: CapOverflow<[i32; CAP]> = CapOverflow::new(CAP + 1);
const TARGET_UNDERFLOW: CapUnderflow<[i32; CAP]> = CapUnderflow::new(CAP - 1);

mod fit_error {
    use super::*;

    mod ensure_can_fit {
        use super::*;

        check_eq!(fits: FitError::ensure_can_fit(&FITS_ITER, CAP, CAP) => Ok(()));
        check_eq!(overflow: FitError::ensure_can_fit(&OVER_ITER, CAP, CAP) 
            => Err(FitError::Overflows(CAP_OVERFLOW)));
        check_eq!(underflow: FitError::ensure_can_fit(&UNDER_ITER, CAP, CAP) 
            => Err(FitError::Underflows(CAP_UNDERFLOW)));

        panics!(bad_iter: FitError::ensure_can_fit(&INVALID_ITERATOR, CAP, CAP) 
            => "Invalid size hint: InvalidSizeHint");
    }

    mod from_target_cap {
        use collection_cap::err::CapError;

        use super::*;

        check_eq!(overflow: FitError::from(TARGET_OVERFLOW) => FitError::Overflows(CAP_OVERFLOW));
        check_eq!(underflow: FitError::from(TARGET_UNDERFLOW) => FitError::Underflows(CAP_UNDERFLOW));
        check_eq!(cap_err_overflow: FitError::from(CapError::Overflow(TARGET_OVERFLOW)) => FitError::Overflows(CAP_OVERFLOW));
        check_eq!(cap_err_underflow: FitError::from(CapError::Underflow(TARGET_UNDERFLOW)) => FitError::Underflows(CAP_UNDERFLOW));
    }
}

mod overflows {
    use super::*;

    check_eq!(new: Overflows::new(CAP + 1, CAP) => CAP_OVERFLOW);
    panics!(panic_new: Overflows::new(CAP, CAP + 1) => "min_size must be greater than max_cap");

    mod ensure_can_fit {
        use super::*;

        check_eq!(fits: Overflows::ensure_can_fit(&FITS_ITER, CAP) => Ok(()));
        check_eq!(overflow: Overflows::ensure_can_fit(&OVER_ITER, CAP) 
            => Err(CAP_OVERFLOW));

        panics!(bad_iter: Overflows::ensure_can_fit(&INVALID_ITERATOR, CAP) 
            => "Invalid size hint: InvalidSizeHint");
    }

    mod ensure_can_fit_into {
        use super::*;

        const CAP_ARRAY_VEC: ArrayVec<i32, CAP> = ArrayVec::new_const();

        check_eq!(fits: Overflows::ensure_can_fit_in(&FITS_ITER, &CAP_ARRAY_VEC) => Ok(()));
        check_eq!(overflow: Overflows::ensure_can_fit_in(&OVER_ITER, &CAP_ARRAY_VEC) 
            => Err(CAP_OVERFLOW));

        panics!(bad_iter: Overflows::ensure_can_fit_in(&INVALID_ITERATOR, &CAP_ARRAY_VEC) 
            => "Invalid size hint: InvalidSizeHint");
    }

    check_eq!(from: Overflows::from(TARGET_OVERFLOW) => CAP_OVERFLOW);
}

mod underflows {
    use super::*;

    check_eq!(new: Underflows::new(CAP - 1, CAP) => CAP_UNDERFLOW);
    panics!(panic_new: Underflows::new(CAP, CAP - 1) => "max_size must be less than min_cap");

    mod ensure_can_fit {
        use super::*;

        check_eq!(fits: Underflows::ensure_can_fit(&FITS_ITER, CAP) => Ok(()));
        check_eq!(underflow: Underflows::ensure_can_fit(&UNDER_ITER, CAP) 
            => Err(CAP_UNDERFLOW));

        panics!(bad_iter: Underflows::ensure_can_fit(&INVALID_ITERATOR, CAP) 
            => "Invalid size hint: InvalidSizeHint");
    }

    mod ensure_fits {
        use super::*;

        check_eq!(fits: Underflows::ensure_fits(&FITS_ITER, CAP) => Ok(()));
        check_eq!(underflow: Underflows::ensure_fits(&UNDER_ITER, CAP) 
            => Err(CAP_UNDERFLOW));
    }

    check_eq!(from: Underflows::from(TARGET_UNDERFLOW) => CAP_UNDERFLOW);
}
