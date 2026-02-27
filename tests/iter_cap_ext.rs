mod common;

use collection_cap::IterCapExt;
use collection_cap::err::{CapError, CapOverflow, CapUnderflow};

use common::consts::*;
use common::{check_eq, panics};

type FixedCap = [i32; CAP];

const TARGET_OVERFLOW: CapOverflow<FixedCap> = CapOverflow::new(CAP + 1);
const TARGET_UNDERFLOW: CapUnderflow<FixedCap> = CapUnderflow::new(CAP - 1);

mod ensure_compatible {
    use super::*;

    check_eq!(compatible: COMPAT_ITER.ensure_compatible::<FixedCap>() => Ok(()));
    check_eq!(overflow: OVER_ITER.ensure_compatible::<FixedCap>() 
        => Err(CapError::Overflow(TARGET_OVERFLOW)));
    check_eq!(underflow: UNDER_ITER.ensure_compatible::<FixedCap>() 
    => Err(CapError::Underflow(TARGET_UNDERFLOW)));

    panics!(bad_iter: INVALID_ITERATOR.ensure_compatible::<FixedCap>() 
    => "Invalid size hint");

    #[test]
    fn dyn_iterator() {
        let iter: &dyn Iterator<Item = i32> = &COMPAT_ITER;
        iter.ensure_compatible::<FixedCap>().expect("Should work for dyn Iterator");
    }
}

const CAP_RANGE: std::ops::Range<usize> = CAP..CAP + 1;

mod ensure_compatible_with {
    use super::*;

    check_eq!(compatible: COMPAT_ITER.ensure_compatible_with(CAP_RANGE) => Ok(()));
    check_eq!(overflow: OVER_ITER.ensure_compatible_with(CAP_RANGE) 
        => Err(COMPAT_ERROR_OVERFLOWS));
    check_eq!(underflow: UNDER_ITER.ensure_compatible_with(CAP_RANGE) 
        => Err(COMPAT_ERROR_UNDERFLOWS));

    panics!(bad_iter: INVALID_ITERATOR.ensure_compatible_with(CAP_RANGE) 
        => "Invalid size hint");
}
