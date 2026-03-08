use core::ops::Not;
use fluent_result::into::{IntoOption, IntoResult};

use crate::err::{
    IntersectError, MaxOverflow, MaxUnderflow, MinOverflow, MinUnderflow, OverlapError, OverlapErrorSpan,
};
use crate::internal::Ok;
use crate::{Capacity, ConstCap, IterExt};

/// Checks if `iter` intersects the `const` [`Capacity`] constraints `MIN` and `MAX`.
///
/// # Errors
///
/// Returns an error if the [`Iterator`]'s size hint indicates it will definitely
/// overflow or underflow the capacity.
pub fn check_const_intersect<MIN, MAX>(iter: &(impl Iterator + ?Sized)) -> Result<(), IntersectError<MIN, MAX>>
where
    MIN: ConstCap<Cap = MIN> + Capacity,
    MAX: ConstCap<Cap = MAX> + Capacity,
{
    match iter.valid_size_hint() {
        (min_size, _) if !MAX::CAP.contains_size(min_size) // fmt
            => MinOverflow::from_parts(min_size, MAX::CAP).into_err()?,
        (_, Some(max_size)) if !MIN::CAP.contains_size(max_size) // fmt
            => MaxUnderflow::from_parts(max_size, MIN::CAP).into_err()?,
        _ => Ok!(),
    }
}

/// Checks if `iter` is guaranteed to overlap the `const` [`Capacity`]
/// constraints `MIN` and `MAX`.
///
/// # Errors
///
/// Returns an error if the [`Iterator`]'s size hint indicates it could
/// potentially overflow or underflow the capacity.
pub fn check_const_overlaps<MIN, MAX>(iter: &(impl Iterator + ?Sized)) -> Result<(), OverlapError<MIN, MAX>>
where
    MIN: ConstCap<Cap = MIN> + Capacity,
    MAX: ConstCap<Cap = MAX> + Capacity,
{
    let (min, max) = iter.valid_size_hint();

    let underflow = MIN::CAP // fmt
        .contains_size(min)
        .not()
        .then(|| MinUnderflow::from_parts(min, MIN::CAP));

    let overflow = match max {
        Some(max) if !MAX::CAP.contains_size(max) // fmt
            => MaxOverflow::from_parts_fixed(max, MAX::CAP).into_some(),
        None => MaxOverflow::unbounded(MAX::CAP).into_some(),
        _ => None,
    };

    match (underflow, overflow) {
        (Some(underflow), Some(overflow)) => OverlapErrorSpan::from_parts(overflow, underflow) //
            .into_err()?,
        (Some(underflow), None) => OverlapError::Underflow(underflow).into_err(),
        (None, Some(overflow)) => OverlapError::Overflow(overflow).into_err(),
        (None, None) => Ok!(),
    }
}
