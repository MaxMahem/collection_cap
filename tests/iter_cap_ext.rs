mod common;

use collection_cap::IterCapExt;
use collection_cap::err::{StaticCapError, StaticCapOverflow, StaticCapUnderflow};

use common::consts::*;
use common::{check_eq, panics};

type FixedCap = [i32; CAP];

mod ensure_compatible {
    use super::*;

    check_eq!(compatible: COMPAT_ITER.ensure_compatible::<FixedCap>() => Ok(()));
    check_eq!(overflow: OVER_ITER.ensure_compatible::<FixedCap>() 
        => Err(StaticCapError::Overflow(StaticCapOverflow::new(OVER_CAP))));
    check_eq!(underflow: UNDER_ITER.ensure_compatible::<FixedCap>() 
        => Err(StaticCapError::Underflow(StaticCapUnderflow::new(UNDER_CAP))));

    panics!(bad_iter: INVALID_ITER.ensure_compatible::<FixedCap>() 
        => "Invalid size hint");

    #[test]
    fn dyn_iterator() {
        let iter: &dyn Iterator<Item = i32> = &COMPAT_ITER;
        iter.ensure_compatible::<FixedCap>().expect("Should work for dyn Iterator");
    }
}

mod ensure_compatible_with {
    use super::*;

    check_eq!(compatible: COMPAT_ITER.ensure_compatible_with(CAP_RANGE) => Ok(()));
    check_eq!(overflow: OVER_ITER.ensure_compatible_with(CAP_RANGE) 
        => Err(CAP_ERROR_OVERFLOW));
    check_eq!(underflow: UNDER_ITER.ensure_compatible_with(CAP_RANGE) 
        => Err(CAP_ERROR_UNDERFLOW));

    panics!(bad_iter: INVALID_ITER.ensure_compatible_with(CAP_RANGE) 
        => "Invalid size hint");
}
