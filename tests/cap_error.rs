mod common;

use collection_cap::err::{TargetCapError, TargetOverflow, TargetUnderflow};

use common::consts::*;
use common::{check_eq, panics};

type FixedCap = [i32; CAP];

const TARGET_OVERFLOW: TargetOverflow<FixedCap> = TargetOverflow::new(CAP + 1);
const TARGET_UNDERFLOW: TargetUnderflow<FixedCap> = TargetUnderflow::new(CAP - 1);

mod target_cap_error {
    use super::*;

    mod ensure_can_fit {
        use super::*;

        check_eq!(fits: TargetCapError::<FixedCap>::ensure_can_fit(&FITS_ITER) => Ok(()));
        check_eq!(overflow: TargetCapError::<FixedCap>::ensure_can_fit(&OVER_ITER) 
            => Err(TargetCapError::Overflow(TARGET_OVERFLOW)));
        check_eq!(underflow: TargetCapError::<FixedCap>::ensure_can_fit(&UNDER_ITER) 
            => Err(TargetCapError::Underflow(TARGET_UNDERFLOW)));

        panics!(bad_iter: TargetCapError::<FixedCap>::ensure_can_fit(&INVALID_ITERATOR) 
            => "Invalid size hint: InvalidSizeHint");
    }

    mod ensure_fits {
        use super::*;

        check_eq!(fits: TargetCapError::<FixedCap>::ensure_fits(&FITS_ITER) => Ok(()));
        check_eq!(overflow: TargetCapError::<FixedCap>::ensure_fits(&OVER_ITER) 
            => Err(TargetCapError::Overflow(TARGET_OVERFLOW)));
        check_eq!(underflow: TargetCapError::<FixedCap>::ensure_fits(&UNDER_ITER) 
            => Err(TargetCapError::Underflow(TARGET_UNDERFLOW)));

        panics!(bad_iter: TargetCapError::<FixedCap>::ensure_fits(&INVALID_ITERATOR) 
            => "Invalid size hint: InvalidSizeHint");
    }
}

mod target_overflow {
    use super::*;

    mod ensure_can_fit {
        use super::*;

        check_eq!(fits: TargetOverflow::<FixedCap>::ensure_can_fit(&FITS_ITER) => Ok(()));
        check_eq!(overflow: TargetOverflow::<FixedCap>::ensure_can_fit(&OVER_ITER) 
            => Err(TARGET_OVERFLOW));

        panics!(bad_iter: TargetOverflow::<FixedCap>::ensure_can_fit(&INVALID_ITERATOR) 
            => "Invalid size hint: InvalidSizeHint");
    }

    mod ensure_fits {
        use super::*;

        check_eq!(fits: TargetOverflow::<FixedCap>::ensure_fits(&FITS_ITER) => Ok(()));
        check_eq!(overflow: TargetOverflow::<FixedCap>::ensure_fits(&OVER_ITER) 
            => Err(TARGET_OVERFLOW));

        panics!(bad_iter: TargetOverflow::<FixedCap>::ensure_fits(&INVALID_ITERATOR) 
            => "Invalid size hint: InvalidSizeHint");
    }
}

mod target_underflow {
    use super::*;

    mod ensure_can_fit {
        use super::*;

        check_eq!(fits: TargetUnderflow::<FixedCap>::ensure_can_fit(&FITS_ITER) => Ok(()));
        check_eq!(underflow: TargetUnderflow::<FixedCap>::ensure_can_fit(&UNDER_ITER) 
            => Err(TARGET_UNDERFLOW));

        panics!(bad_iter: TargetUnderflow::<FixedCap>::ensure_can_fit(&INVALID_ITERATOR) 
            => "Invalid size hint: InvalidSizeHint");
    }

    mod ensure_fits {
        use super::*;

        check_eq!(fits: TargetUnderflow::<FixedCap>::ensure_fits(&FITS_ITER) => Ok(()));
        check_eq!(underflow: TargetUnderflow::<FixedCap>::ensure_fits(&UNDER_ITER) 
            => Err(TARGET_UNDERFLOW));

        panics!(bad_iter: TargetUnderflow::<FixedCap>::ensure_fits(&INVALID_ITERATOR) 
            => "Invalid size hint: InvalidSizeHint");
    }
}
