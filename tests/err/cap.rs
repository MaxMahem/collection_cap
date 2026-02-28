use collection_cap::err::{CapError, CapOverflow, CapUnderflow};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

type FixedCap = [i32; CAP];

const TARGET_OVERFLOW: CapOverflow<FixedCap> = CapOverflow::new(CAP + 1);
const TARGET_UNDERFLOW: CapUnderflow<FixedCap> = CapUnderflow::new(CAP - 1);

mod cap_error {
    use super::*;

    mod ensure_compatible {
        use super::*;

        check_eq!(compatible: CapError::<FixedCap>::ensure_compatible(&COMPAT_ITER) => Ok(()));
        check_eq!(overflow: CapError::<FixedCap>::ensure_compatible(&OVER_ITER) 
            => Err(CapError::Overflow(TARGET_OVERFLOW)));
        check_eq!(underflow: CapError::<FixedCap>::ensure_compatible(&UNDER_ITER) 
            => Err(CapError::Underflow(TARGET_UNDERFLOW)));

        panics!(bad_iter: CapError::<FixedCap>::ensure_compatible(&INVALID_ITER) 
            => "Invalid size hint");
    }
}

mod cap_overflow {
    use super::*;

    check_eq!(new: CapOverflow::<FixedCap>::new(CAP + 1) => TARGET_OVERFLOW);
    panics!(panic: CapOverflow::<FixedCap>::new(CAP) => "min_size must be > MAX_CAP");
    check_eq!(min_size: TARGET_OVERFLOW.min_size() => CAP + 1);

    mod ensure_compatible {
        use super::*;

        check_eq!(compatible: CapOverflow::<FixedCap>::ensure_compatible(&COMPAT_ITER) => Ok(()));
        check_eq!(overflow: CapOverflow::<FixedCap>::ensure_compatible(&OVER_ITER) 
            => Err(TARGET_OVERFLOW));

        panics!(bad_iter: CapOverflow::<FixedCap>::ensure_compatible(&INVALID_ITER) 
            => "Invalid size hint");
    }
}

mod cap_underflow {
    use super::*;

    check_eq!(new: CapUnderflow::<FixedCap>::new(CAP - 1) => TARGET_UNDERFLOW);
    panics!(panic: CapUnderflow::<FixedCap>::new(CAP) => "max_size must be < MIN_CAP");
    check_eq!(max_size: TARGET_UNDERFLOW.max_size() => CAP - 1);

    mod ensure_compatible {
        use super::*;

        check_eq!(compatible: CapUnderflow::<FixedCap>::ensure_compatible(&COMPAT_ITER) => Ok(()));
        check_eq!(underflow: CapUnderflow::<FixedCap>::ensure_compatible(&UNDER_ITER) 
            => Err(TARGET_UNDERFLOW));

        panics!(bad_iter: CapUnderflow::<FixedCap>::ensure_compatible(&INVALID_ITER) 
            => "Invalid size hint");
    }
}
