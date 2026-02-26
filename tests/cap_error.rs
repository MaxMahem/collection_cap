mod common;

use collection_cap::err::{CapError, CapOverflow, CapUnderflow};

use common::consts::*;
use common::{check_eq, panics};

type FixedCap = [i32; CAP];

const TARGET_OVERFLOW: CapOverflow<FixedCap> = CapOverflow::new(CAP + 1);
const TARGET_UNDERFLOW: CapUnderflow<FixedCap> = CapUnderflow::new(CAP - 1);

mod cap_error {
    use super::*;

    mod ensure_can_fit {
        use super::*;

        check_eq!(fits: CapError::<FixedCap>::ensure_can_fit(&FITS_ITER) => Ok(()));
        check_eq!(overflow: CapError::<FixedCap>::ensure_can_fit(&OVER_ITER) 
            => Err(CapError::Overflow(TARGET_OVERFLOW)));
        check_eq!(underflow: CapError::<FixedCap>::ensure_can_fit(&UNDER_ITER) 
            => Err(CapError::Underflow(TARGET_UNDERFLOW)));

        panics!(bad_iter: CapError::<FixedCap>::ensure_can_fit(&INVALID_ITERATOR) 
            => "Invalid size hint: InvalidSizeHint");
    }

    mod ensure_fits {
        use super::*;

        check_eq!(fits: CapError::<FixedCap>::ensure_fits(&FITS_ITER) => Ok(()));
        check_eq!(overflow: CapError::<FixedCap>::ensure_fits(&OVER_ITER) 
            => Err(CapError::Overflow(TARGET_OVERFLOW)));
        check_eq!(underflow: CapError::<FixedCap>::ensure_fits(&UNDER_ITER) 
            => Err(CapError::Underflow(TARGET_UNDERFLOW)));

        panics!(bad_iter: CapError::<FixedCap>::ensure_fits(&INVALID_ITERATOR) 
            => "Invalid size hint: InvalidSizeHint");
    }
}

mod target_overflow {
    use super::*;

    mod ensure_can_fit {
        use super::*;

        check_eq!(fits: CapOverflow::<FixedCap>::ensure_can_fit(&FITS_ITER) => Ok(()));
        check_eq!(overflow: CapOverflow::<FixedCap>::ensure_can_fit(&OVER_ITER) 
            => Err(TARGET_OVERFLOW));

        panics!(bad_iter: CapOverflow::<FixedCap>::ensure_can_fit(&INVALID_ITERATOR) 
            => "Invalid size hint: InvalidSizeHint");
    }

    mod ensure_fits {
        use super::*;

        check_eq!(fits: CapOverflow::<FixedCap>::ensure_fits(&FITS_ITER) => Ok(()));
        check_eq!(overflow: CapOverflow::<FixedCap>::ensure_fits(&OVER_ITER) 
            => Err(TARGET_OVERFLOW));

        panics!(bad_iter: CapOverflow::<FixedCap>::ensure_fits(&INVALID_ITERATOR) 
            => "Invalid size hint: InvalidSizeHint");
    }
}

mod target_underflow {
    use super::*;

    mod ensure_can_fit {
        use super::*;

        check_eq!(fits: CapUnderflow::<FixedCap>::ensure_can_fit(&FITS_ITER) => Ok(()));
        check_eq!(underflow: CapUnderflow::<FixedCap>::ensure_can_fit(&UNDER_ITER) 
            => Err(TARGET_UNDERFLOW));

        panics!(bad_iter: CapUnderflow::<FixedCap>::ensure_can_fit(&INVALID_ITERATOR) 
            => "Invalid size hint: InvalidSizeHint");
    }

    mod ensure_fits {
        use super::*;

        check_eq!(fits: CapUnderflow::<FixedCap>::ensure_fits(&FITS_ITER) => Ok(()));
        check_eq!(underflow: CapUnderflow::<FixedCap>::ensure_fits(&UNDER_ITER) 
            => Err(TARGET_UNDERFLOW));

        panics!(bad_iter: CapUnderflow::<FixedCap>::ensure_fits(&INVALID_ITERATOR) 
            => "Invalid size hint: InvalidSizeHint");
    }
}
