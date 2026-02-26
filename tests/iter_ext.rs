mod common;

use collection_cap::IterCapExt;
use collection_cap::err::{CapError, CapOverflow, CapUnderflow};

use common::consts::*;
use common::{check_eq, panics};

type FixedCap = [i32; CAP];

const TARGET_OVERFLOW: CapOverflow<FixedCap> = CapOverflow::new(CAP + 1);
const TARGET_UNDERFLOW: CapUnderflow<FixedCap> = CapUnderflow::new(CAP - 1);

mod ensure_can_fit {
    use super::*;

    check_eq!(fits: FITS_ITER.ensure_can_fit::<FixedCap>() => Ok(()));
    check_eq!(overflow: OVER_ITER.ensure_can_fit::<FixedCap>() 
        => Err(CapError::Overflow(TARGET_OVERFLOW)));
    check_eq!(underflow: UNDER_ITER.ensure_can_fit::<FixedCap>() 
    => Err(CapError::Underflow(TARGET_UNDERFLOW)));

    panics!(bad_iter: INVALID_ITERATOR.ensure_can_fit::<FixedCap>() 
    => "Invalid size hint: InvalidSizeHint");

    #[test]
    fn dyn_iterator() {
        let iter: &dyn Iterator<Item = i32> = &FITS_ITER;
        iter.ensure_can_fit::<FixedCap>().expect("Should work for dyn Iterator");
    }
}

mod ensure_fits {
    use super::*;

    check_eq!(fits: FITS_ITER.ensure_fits::<FixedCap>() => Ok(()));
    check_eq!(overflow: OVER_ITER.ensure_fits::<FixedCap>() 
        => Err(CapError::Overflow(TARGET_OVERFLOW)));
    check_eq!(underflow: UNDER_ITER.ensure_fits::<FixedCap>() 
        => Err(CapError::Underflow(TARGET_UNDERFLOW)));

    panics!(bad_iter: INVALID_ITERATOR.ensure_fits::<FixedCap>() 
        => "Invalid size hint: InvalidSizeHint");
}
