use core::ops::Not;
use fluent_result::into::{IntoOption, IntoResult};

use crate::err::{CompatError, FitError, FitErrorSpan, MaxOverflow, MaxUnderflow, MinOverflow, MinUnderflow};
use crate::internal::Ok;
use crate::{Capacity, IterExt, StaticCap};

/// Checks if `iter` is compatible with the static capacity constraint `CAP`.
///
/// # Errors
///
/// Returns an error if the iterator's size hint indicates it will definitely overflow or underflow the capacity.
pub fn check_static_compatibility<MIN, MAX>(iter: &(impl Iterator + ?Sized)) -> Result<(), CompatError<MIN, MAX>>
where
    MIN: StaticCap<Cap = MIN> + Capacity,
    MAX: StaticCap<Cap = MAX> + Capacity,
{
    match iter.valid_size_hint() {
        (min_size, _) if !MAX::CAP.contains_size(min_size) // fmt
            => MinOverflow::from_parts(min_size, MAX::CAP).into_err()?,
        (_, Some(max_size)) if !MIN::CAP.contains_size(max_size) // fmt
            => MaxUnderflow::from_parts(max_size, MIN::CAP).into_err()?,
        _ => Ok!(),
    }
}

/// Checks if `iter` is guaranteed to fit within the static capacity constraint `C`.
///
/// # Errors
///
/// Returns an error if the iterator's size hint indicates it could potentially overflow or underflow the capacity.
pub fn check_static_fit<MIN, MAX>(iter: &(impl Iterator + ?Sized)) -> Result<(), FitError<MIN, MAX>>
where
    MIN: StaticCap<Cap = MIN> + Capacity,
    MAX: StaticCap<Cap = MAX> + Capacity,
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
        (Some(underflow), Some(overflow)) => FitErrorSpan::from_parts(overflow, underflow) //
            .into_err()?,
        (Some(underflow), None) => FitError::Underflow(underflow).into_err(),
        (None, Some(overflow)) => FitError::Overflow(overflow).into_err(),
        (None, None) => Ok!(),
    }
}
