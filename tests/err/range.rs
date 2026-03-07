use crate::common::consts::*;
use crate::common::{check_eq, panics};
use collection_cap::err::{EmptyRange, FromRangeError, InvalidRange};

pub const INVALID_RANGE_ERROR: InvalidRange = InvalidRange::new(CAP, UNDER_CAP);

mod invalid_range {
    use super::*;

    check_eq!(new: InvalidRange::new(CAP, UNDER_CAP) => INVALID_RANGE_ERROR);
    check_eq!(start: INVALID_RANGE_ERROR.start() => CAP);
    check_eq!(end: INVALID_RANGE_ERROR.end() => UNDER_CAP);

    panics!(valid_range: InvalidRange::new(CAP, CAP) => "Range is valid");
}

mod from_range_error {
    use super::*;

    check_eq!(empty: FromRangeError::from(EmptyRange) => FromRangeError::EMPTY);
    check_eq!(invalid: FromRangeError::from(INVALID_RANGE_ERROR) 
        => FromRangeError::InvalidRange(INVALID_RANGE_ERROR));
}
