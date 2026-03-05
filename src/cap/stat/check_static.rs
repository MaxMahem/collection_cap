use core::ops::{Not, RangeBounds};
use fluent_result::into::{IntoOption, IntoResult};
use tap::Pipe;

use crate::err::{CompatError, FitError, FitErrorSpan, MaxOverflow, MaxUnderflow, MinOverflow, MinUnderflow};
use crate::internal::Ok;
use crate::{Capacity, IterExt, StaticCap};

/// Checks if `iter` is compatible with the static capacity constraint `C`.
///
/// # Errors
///
/// Returns an error if the iterator's size hint indicates it will definitely overflow or underflow the capacity.
pub fn check_static_compatibility<CAP, I>(iter: &I) -> Result<(), CompatError<CAP::Min, CAP::Max>>
where
    CAP: StaticCap<Cap = CAP> + Capacity,
    I: Iterator + ?Sized,
{
    match iter.valid_size_hint() {
        (min_size, _) if !CAP::CAP.max_cap().contains(&min_size) // fmt
            => MinOverflow::from_parts(min_size, CAP::CAP.max_cap()) //
                .pipe(CompatError::Overflow)
                .into_err(),
        (_, Some(max_size)) if !CAP::CAP.min_cap().contains(&max_size) // fmt
            => MaxUnderflow::from_parts(max_size, CAP::CAP.min_cap()) //
                .pipe(CompatError::Underflow)
                .into_err(),
        _ => Ok!(),
    }
}

/// Checks if `iter` is guaranteed to fit within the static capacity constraint `C`.
///
/// # Errors
///
/// Returns an error if the iterator's size hint indicates it could potentially overflow or underflow the capacity.
pub fn check_static_fit<CAP, I>(iter: &I) -> Result<(), FitError<CAP::Min, CAP::Max>>
where
    CAP: StaticCap<Cap = CAP> + Capacity,
    I: Iterator + ?Sized,
{
    let (min, max) = iter.valid_size_hint();

    let underflow = CAP::CAP // fmt
        .min_cap()
        .contains(&min)
        .not()
        .then(|| MinUnderflow::from_parts(min, CAP::CAP.min_cap()));

    let overflow = match max {
        Some(max) if !CAP::CAP.max_cap().contains(&max) // fmt
            => MaxOverflow::from_parts(max, CAP::CAP.max_cap()).into_some(),
        None => Some(MaxOverflow::from_parts_unbounded(CAP::CAP.max_cap())),
        _ => None,
    };

    match (underflow, overflow) {
        (Some(underflow), Some(overflow)) => FitErrorSpan::from_parts(overflow, underflow) //
            .pipe(FitError::Both)
            .into_err(),
        (Some(underflow), None) => FitError::Underflow(underflow).into_err(),
        (None, Some(overflow)) => FitError::Overflow(overflow).into_err(),
        (None, None) => Ok!(),
    }
}
