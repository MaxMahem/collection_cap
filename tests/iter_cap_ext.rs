mod common;

use crate::common::consts::iter::*;
use crate::common::consts::*;
use crate::common::{check_eq, panics};

use collection_cap::IterCapExt;
use collection_cap::cap::{MaxCapVal, MinCapVal, StaticMaxCap, StaticMinCap};
use collection_cap::err::{CompatError, MaxUnderflow, MinOverflow};
use collection_cap::err::{FitError, FitErrorSpan, MaxOverflow, MinUnderflow};

type FixedCap = [i32; CAP];

mod ensure_compatible {
    use super::*;

    const MAX_UNDERFLOW: MaxUnderflow<StaticMinCap<CAP>> = MaxUnderflow::<StaticMinCap<CAP>>::new(UNDER_CAP);
    const MIN_OVERFLOW: MinOverflow<StaticMaxCap<CAP>> = MinOverflow::<StaticMaxCap<CAP>>::new(OVER_CAP);

    check_eq!(compatible: COMPAT_ITER.ensure_compatible::<FixedCap>() => Ok(()));
    check_eq!(underflow: UNDER_ITER.ensure_compatible::<FixedCap>()
        => Err(CompatError::Underflow(MAX_UNDERFLOW)));
    check_eq!(overflow: OVER_ITER.ensure_compatible::<FixedCap>()
        => Err(CompatError::Overflow(MIN_OVERFLOW)));

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

    const MIN_CAP: MinCapVal = MinCapVal(CAP);
    const MAX_CAP: MaxCapVal = MaxCapVal(CAP);
    const MAX_UNDERFLOW: MaxUnderflow<MinCapVal> = MaxUnderflow::<MinCapVal>::new(UNDER_CAP, MIN_CAP);
    const MIN_OVERFLOW: MinOverflow<MaxCapVal> = MinOverflow::<MaxCapVal>::new(OVER_CAP, MAX_CAP);

    check_eq!(compatible: COMPAT_ITER.ensure_compatible_with(CAP_RANGE) => Ok(()));
    check_eq!(overflow: OVER_ITER.ensure_compatible_with(CAP_RANGE) 
        => Err(CompatError::Overflow(MIN_OVERFLOW)));
    check_eq!(underflow: UNDER_ITER.ensure_compatible_with(CAP_RANGE) 
        => Err(CompatError::Underflow(MAX_UNDERFLOW)));

    panics!(bad_iter: INVALID_ITER.ensure_compatible_with(CAP_RANGE) 
        => "Invalid size hint");
}

mod ensure_fit {
    use super::*;

    const MIN_UNDERFLOW: MinUnderflow<StaticMinCap<CAP>> = MinUnderflow::<StaticMinCap<CAP>>::new(UNDER_CAP);
    const MAX_OVERFLOW: MaxOverflow<StaticMaxCap<CAP>> = MaxOverflow::<StaticMaxCap<CAP>>::fixed(OVER_CAP);
    const FIT_ERROR_SPAN: FitErrorSpan<StaticMinCap<CAP>, StaticMaxCap<CAP>> =
        FitErrorSpan::new(MAX_OVERFLOW, MIN_UNDERFLOW);

    check_eq!(fit: COMPAT_ITER.ensure_fit::<FixedCap>() => Ok(()));
    check_eq!(underflow: UNDER_ITER.ensure_fit::<FixedCap>()
        => Err(FitError::Underflow(MIN_UNDERFLOW)));
    check_eq!(overflow: OVER_ITER.ensure_fit::<FixedCap>()
        => Err(FitError::Overflow(MAX_OVERFLOW)));
    check_eq!(both: BOTH_ITER.ensure_fit::<FixedCap>()
        => Err(FitError::Both(FIT_ERROR_SPAN)));

    panics!(bad_iter: INVALID_ITER.ensure_fit::<FixedCap>() 
        => "Invalid size hint");

    #[test]
    fn dyn_iterator() {
        let iter: &dyn Iterator<Item = i32> = &COMPAT_ITER;
        iter.ensure_fit::<FixedCap>().expect("Should work for dyn Iterator");
    }
}

mod ensure_fits_into {
    use super::*;

    const MIN_CAP: MinCapVal = MinCapVal(CAP);
    const MAX_CAP: MaxCapVal = MaxCapVal(CAP);
    const MIN_UNDERFLOW: MinUnderflow<MinCapVal> = MinUnderflow::<MinCapVal>::new(UNDER_CAP, MIN_CAP);
    const MAX_OVERFLOW: MaxOverflow<MaxCapVal> = MaxOverflow::<MaxCapVal>::fixed(OVER_CAP, MAX_CAP);
    const FIT_ERROR_SPAN: FitErrorSpan<MinCapVal, MaxCapVal> = FitErrorSpan::new(MAX_OVERFLOW, MIN_UNDERFLOW);

    check_eq!(fit: COMPAT_ITER.ensure_fits_into(CAP_RANGE) => Ok(()));
    check_eq!(overflow: OVER_ITER.ensure_fits_into(CAP_RANGE) 
        => Err(FitError::Overflow(MAX_OVERFLOW)));
    check_eq!(underflow: UNDER_ITER.ensure_fits_into(CAP_RANGE) 
        => Err(FitError::Underflow(MIN_UNDERFLOW)));
    check_eq!(both: BOTH_ITER.ensure_fits_into(CAP_RANGE) 
        => Err(FitError::Both(FIT_ERROR_SPAN)));

    panics!(bad_iter: INVALID_ITER.ensure_fits_into(CAP_RANGE) 
        => "Invalid size hint");
}
