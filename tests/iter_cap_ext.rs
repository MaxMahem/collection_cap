mod common;

use collection_cap::IterCapExt;
use collection_cap::cap::{StaticMaxCap, StaticMinCap};
use collection_cap::err::{CompatError, MaxUnderflow, MinOverflow};
use common::consts::*;
use common::{check_eq, panics};

type FixedCap = [i32; base::CAP];

mod ensure_compatible {
    use super::*;

    check_eq!(compatible: iter::COMPAT_ITER.ensure_compatible::<FixedCap>() => Ok(()));
    check_eq!(underflow: iter::UNDER_ITER.ensure_compatible::<FixedCap>()
        => Err(CompatError::Underflow(MaxUnderflow::<StaticMinCap<{ base::CAP }>>::new(base::UNDER_CAP))));
    check_eq!(overflow: iter::OVER_ITER.ensure_compatible::<FixedCap>()
        => Err(CompatError::Overflow(MinOverflow::<StaticMaxCap<{ base::CAP }>>::new(base::OVER_CAP))));

    panics!(bad_iter: iter::INVALID_ITER.ensure_compatible::<FixedCap>() 
        => "Invalid size hint");

    #[test]
    fn dyn_iterator() {
        let iter: &dyn Iterator<Item = i32> = &iter::COMPAT_ITER;
        iter.ensure_compatible::<FixedCap>().expect("Should work for dyn Iterator");
    }
}

mod ensure_compatible_with {
    use super::*;

    check_eq!(compatible: iter::COMPAT_ITER.ensure_compatible_with(base::CAP_RANGE) => Ok(()));
    check_eq!(overflow: iter::OVER_ITER.ensure_compatible_with(base::CAP_RANGE) 
        => Err(err_val_compat::OVERFLOW));
    check_eq!(underflow: iter::UNDER_ITER.ensure_compatible_with(base::CAP_RANGE) 
        => Err(err_val_compat::UNDERFLOW));

    panics!(bad_iter: iter::INVALID_ITER.ensure_compatible_with(base::CAP_RANGE) 
        => "Invalid size hint");
}
