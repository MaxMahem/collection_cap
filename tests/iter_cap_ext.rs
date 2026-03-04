mod common;

use collection_cap::IterCapExt;
use collection_cap::cap::{StaticMaxCap, StaticMinCap};
use collection_cap::err::{CompatError, MaxUnderflow, MinOverflow};
use common::consts::*;
use common::{check_eq, panics};

type FixedCap = [i32; CAP];

mod ensure_compatible {
    use super::*;

    check_eq!(compatible: COMPAT_ITER.ensure_compatible::<FixedCap>() => Ok(()));
    check_eq!(underflow: UNDER_ITER.ensure_compatible::<FixedCap>()
        => Err(CompatError::Underflow(MaxUnderflow::<StaticMinCap<CAP>>::new(UNDER_CAP))));
    check_eq!(overflow: OVER_ITER.ensure_compatible::<FixedCap>()
        => Err(CompatError::Overflow(MinOverflow::<StaticMaxCap<CAP>>::new(OVER_CAP))));

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
