#![allow(unused_imports)]
use crate::common::consts::*;
use super::*;

check_eq!(capacity: MIN_MAX_CAP.capacity() => MIN_MAX_CAP);
check_eq!(new: MinMaxCapVal::new(base::CAP, base::CAP) => MIN_MAX_CAP);
check_eq!(zero: MinMaxCapVal::ZERO => MinMaxCapVal::new(0, 0));
check_eq!(min_val: MIN_MAX_CAP.min() => MinCapVal(base::CAP));
check_eq!(max_val: MIN_MAX_CAP.max() => MaxCapVal(base::CAP));
check_eq!(min_cap: MIN_MAX_CAP.min_cap() => MIN_CAP);
check_eq!(max_cap: MIN_MAX_CAP.max_cap() => MAX_CAP);

mod from {
    use super::*;

    check_eq!(exact: MinMaxCapVal::from(EXACT_CAP) => MIN_MAX_CAP);
    check_eq!(static_cap: MinMaxCapVal::from(StaticMinMaxCap::<{ base::CAP }, { base::CAP }>) => MIN_MAX_CAP);
    check_eq!(static_cap_exact: MinMaxCapVal::from(StaticExactCap::<{ base::CAP }>) => MIN_MAX_CAP);
}

mod try_from_range {
    use super::*;

    check_eq!(valid: MinMaxCapVal::try_from(base::CAP..base::CAP + 1) => Ok(MIN_MAX_CAP));
    check_eq!(empty: MinMaxCapVal::try_from(base::CAP..base::CAP) => Err(RangeError::Empty(EmptyRange)));
    check_eq!(invalid: MinMaxCapVal::try_from(core::ops::Range { start: base::CAP, end: base::CAP - 1 }) => Err(RangeError::InvalidRange(InvalidRange)));
    check_eq!(inclusive_valid: MinMaxCapVal::try_from(base::CAP..=base::CAP) => Ok(MIN_MAX_CAP));
    check_eq!(inclusive_invalid: MinMaxCapVal::try_from(core::ops::RangeInclusive::new(base::CAP, base::CAP - 1)) => Err(InvalidRange));
}

check_eq!(eq: MIN_MAX_CAP == EXACT_CAP => true);
check_eq!(ne: MIN_MAX_CAP != EXACT_CAP => false);

mod range_bounds {
    use super::*;

    check_eq!(start_bound: MIN_MAX_CAP.start_bound() => Included(&base::CAP));
    check_eq!(end_bound: MIN_MAX_CAP.end_bound() => Included(&base::CAP));
}

mod check_compat {
    use super::*;

    check_eq!(compatible: MIN_MAX_CAP.check_compatibility(&iter::COMPAT_ITER) => Ok(()));
    check_eq!(overflow: MIN_MAX_CAP.check_compatibility(&iter::OVER_ITER) => Err(err_val_compat::OVERFLOW));
    check_eq!(underflow: MIN_MAX_CAP.check_compatibility(&iter::UNDER_ITER) => Err(err_val_compat::UNDERFLOW));

    panics!(bad_iter: MIN_MAX_CAP.check_compatibility(&iter::INVALID_ITER) => "Invalid size hint");
    panics!(invalid_range: MinMaxCapVal::new(base::CAP, base::CAP - 1) => "Invalid range (start > end)");
}

mod check_fit {
    use super::*;

    check_eq!(compatible: MIN_MAX_CAP.check_fit(&iter::COMPAT_ITER) => Ok(()));
    check_eq!(underflow: MIN_MAX_CAP.check_fit(&iter::UNDER_ITER) => Err(err_val_fit::UNDERFLOW));
    check_eq!(overflow: MIN_MAX_CAP.check_fit(&iter::OVER_ITER) => Err(err_val_fit::OVERFLOW));
    check_eq!(overflow_unbounded: MIN_MAX_CAP.check_fit(&iter::OVER_ITER_UNBOUNDED) => Err(FitError::Overflow(MaxOverflow::unbounded(MAX_CAP))));
    check_eq!(both: MIN_MAX_CAP.check_fit(&iter::BOTH_ITER) => Err(err_val_fit::BOTH));

    panics!(bad_iter: MIN_MAX_CAP.check_fit(&iter::INVALID_ITER) => "Invalid size hint");
}
