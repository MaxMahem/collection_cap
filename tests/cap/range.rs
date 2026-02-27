use std::ops::{Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive};

use collection_cap::ValConstraint;

use crate::common::consts::*;
use crate::common::{check_eq, panics};

mod range_to {
    use super::*;

    const CAP_RANGE: RangeTo<usize> = ..CAP + 1;
    const EMPTY_RANGE: RangeTo<usize> = ..0;

    check_eq!(fits: CAP_RANGE.check_if_can_fit(&FITS_ITER) => Ok(()));
    check_eq!(overflow: CAP_RANGE.check_if_can_fit(&OVER_ITER)
            => Err(CAP_OVERFLOWS));

    panics!(empty: EMPTY_RANGE.check_if_can_fit(&FITS_ITER)
            => "capacity constraint range must not be empty");
}

mod range_to_inclusive {
    use super::*;

    const CAP_RANGE: RangeToInclusive<usize> = ..=CAP;

    check_eq!(fits: CAP_RANGE.check_if_can_fit(&FITS_ITER) => Ok(()));
    check_eq!(overflow: CAP_RANGE.check_if_can_fit(&OVER_ITER)
            => Err(CAP_OVERFLOWS));
}

mod range_from {
    use super::*;

    const CAP_RANGE: RangeFrom<usize> = CAP..;

    check_eq!(fits: CAP_RANGE.check_if_can_fit(&FITS_ITER) => Ok(()));
    check_eq!(underflow: CAP_RANGE.check_if_can_fit(&UNDER_ITER)
            => Err(CAP_UNDERFLOWS));
}

mod range_open {
    use super::*;

    const CAP_RANGE: Range<usize> = CAP..CAP + 1;
    const EMPTY_RANGE: Range<usize> = CAP..CAP;
    const INVALID_RANGE: Range<usize> = Range { start: CAP, end: CAP - 1 };

    check_eq!(fits: CAP_RANGE.check_if_can_fit(&FITS_ITER) => Ok(()));
    check_eq!(overflow: CAP_RANGE.check_if_can_fit(&OVER_ITER)
            => Err(FIT_ERROR_OVERFLOWS));
    check_eq!(underflow: CAP_RANGE.check_if_can_fit(&UNDER_ITER)
            => Err(FIT_ERROR_UNDERFLOWS));

    panics!(empty: EMPTY_RANGE.check_if_can_fit(&FITS_ITER)
            => "range must not be empty");

    panics!(invalid: INVALID_RANGE.check_if_can_fit(&FITS_ITER)
            => "invalid range (start > end)");
}

mod range_inclusive {
    use super::*;

    const CAP_RANGE: RangeInclusive<usize> = CAP..=CAP;
    const INVALID_RANGE: RangeInclusive<usize> = RangeInclusive::new(CAP, CAP - 1);

    check_eq!(fits: CAP_RANGE.check_if_can_fit(&FITS_ITER) => Ok(()));
    check_eq!(overflow: CAP_RANGE.check_if_can_fit(&OVER_ITER)
            => Err(FIT_ERROR_OVERFLOWS));
    check_eq!(underflow: CAP_RANGE.check_if_can_fit(&UNDER_ITER)
            => Err(FIT_ERROR_UNDERFLOWS));

    panics!(invalid: INVALID_RANGE.check_if_can_fit(&FITS_ITER)
            => "invalid range (start > end)");
}

mod range_full {
    use super::*;

    check_eq!(fits: (..).check_if_can_fit(&FITS_ITER) => Ok(()));
    check_eq!(fits_over: (..).check_if_can_fit(&OVER_ITER) => Ok(()));
    check_eq!(fits_under: (..).check_if_can_fit(&UNDER_ITER) => Ok(()));
}
