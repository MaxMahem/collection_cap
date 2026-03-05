use core::ops::{Bound, Not, RangeBounds};
use derive_more::Debug;

use fluent_result::into::{IntoOption, IntoResult};
use tap::Pipe;

use crate::cap::{ExactCapVal, MaxCapVal, MinCapVal, MinMaxCapVal, UnboundedCap};
use crate::err::{CompatError, FitError, FitErrorSpan, MaxOverflow, MaxUnderflow, MinOverflow, MinUnderflow};
use crate::internal::{Sealed, assert_then};
use crate::{Capacity, IterExt, StaticCap};

/// A static minimum capacity constraint.
///
/// # Type Parameters
///
/// * `MIN`: The inclusive minimum size of the constraint.
#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
#[debug("StaticMinCap<{MIN}>")]
pub struct StaticMinCap<const MIN: usize>;

impl<const MIN: usize> StaticCap for StaticMinCap<MIN> {
    type Cap = Self;

    const CAP: Self::Cap = Self;
}

impl<const MIN: usize> Sealed for StaticMinCap<MIN> {}

impl<const MIN: usize> RangeBounds<usize> for StaticMinCap<MIN> {
    fn start_bound(&self) -> Bound<&usize> {
        Bound::Included(&MIN)
    }

    fn end_bound(&self) -> Bound<&usize> {
        Bound::Unbounded
    }
}

impl<const MIN: usize> Capacity for StaticMinCap<MIN> {
    type Error = MaxUnderflow<Self>;
    type FitError = MinUnderflow<Self>;
    type Min = Self;
    type Max = UnboundedCap;

    fn min_cap(&self) -> Self::Min {
        *self
    }

    fn max_cap(&self) -> Self::Max {
        UnboundedCap
    }

    fn check_compatibility<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        match iter.valid_size_hint() {
            (_, Some(max)) if !self.contains(&max) // fmt
                => MaxUnderflow::from_parts(max, Self).into_err(),
            _ => Ok(()),
        }
    }

    fn check_fit<I>(&self, iter: &I) -> Result<(), Self::FitError>
    where
        I: Iterator + ?Sized,
    {
        match iter.valid_size_hint() {
            (min, _) if !self.contains(&min) // fmt
                => MinUnderflow::from_parts(min, Self).into_err(),
            _ => Ok(()),
        }
    }
}

impl<const MIN: usize> From<StaticMinCap<MIN>> for MinCapVal {
    fn from(_value: StaticMinCap<MIN>) -> Self {
        Self(MIN)
    }
}

/// A static maximum capacity constraint.
///
/// # Type Parameters
///
/// * `MAX`: The inclusive maximum size of the constraint.
#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
#[debug("StaticMaxCap<{MAX}>")]
pub struct StaticMaxCap<const MAX: usize>;

impl<const MAX: usize> StaticCap for StaticMaxCap<MAX> {
    type Cap = Self;

    const CAP: Self::Cap = Self;
}

impl<const MAX: usize> Sealed for StaticMaxCap<MAX> {}

impl<const MAX: usize> RangeBounds<usize> for StaticMaxCap<MAX> {
    fn start_bound(&self) -> Bound<&usize> {
        Bound::Unbounded
    }

    fn end_bound(&self) -> Bound<&usize> {
        Bound::Included(&MAX)
    }
}

impl<const MAX: usize> Capacity for StaticMaxCap<MAX> {
    type Error = MinOverflow<Self>;
    type FitError = MaxOverflow<Self>;
    type Min = UnboundedCap;
    type Max = Self;

    fn min_cap(&self) -> Self::Min {
        UnboundedCap
    }

    fn max_cap(&self) -> Self::Max {
        *self
    }

    fn check_compatibility<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        match iter.valid_size_hint() {
            (min_size, _) if !self.contains(&min_size) // fmt
                => MinOverflow::from_parts(min_size, Self).into_err(),
            _ => Ok(()),
        }
    }

    fn check_fit<I>(&self, iter: &I) -> Result<(), Self::FitError>
    where
        I: Iterator + ?Sized,
    {
        match iter.valid_size_hint() {
            (_, Some(max)) if !self.contains(&max) // fmt
                => MaxOverflow::from_parts(max, Self).into_err(),
            (_, None) => Err(MaxOverflow::<Self>::UNBOUNDED),
            _ => Ok(()),
        }
    }
}

impl<const MAX: usize> From<StaticMaxCap<MAX>> for MaxCapVal {
    fn from(_value: StaticMaxCap<MAX>) -> Self {
        Self(MAX)
    }
}

/// Checks if `iter` is compatible with the static capacity constraint `C`.
fn check_static_compatibility<CAP, I>(iter: &I) -> Result<(), CompatError<CAP::Min, CAP::Max>>
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
        _ => Ok(()),
    }
}

/// Checks if `iter` is guaranteed to fit within the static capacity constraint `C`.
fn check_static_fit<CAP, I>(iter: &I) -> Result<(), FitError<CAP::Min, CAP::Max>>
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
        (None, None) => Ok(()),
    }
}

/// A static minimum and maximum capacity constraint.
///
/// If `MIN == MAX`, then consider using [`StaticExactCap`] instead.
///
/// # Type Parameters
///
/// * `MIN`: The inclusive minimum size of the constraint.
/// * `MAX`: The inclusive maximum size of the constraint.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[debug("StaticMinMaxCap<{MIN}, {MAX}>")]
pub struct StaticMinMaxCap<const MIN: usize, const MAX: usize>;

impl<const MIN: usize, const MAX: usize> StaticCap for StaticMinMaxCap<MIN, MAX> {
    type Cap = Self;

    const CAP: Self::Cap = assert_then!(MIN <= MAX => Self, "StaticMinMaxCap: MIN must be <= MAX");
}

impl<const MIN: usize, const MAX: usize> Sealed for StaticMinMaxCap<MIN, MAX> {}

impl<const MIN: usize, const MAX: usize> RangeBounds<usize> for StaticMinMaxCap<MIN, MAX> {
    fn start_bound(&self) -> Bound<&usize> {
        Bound::Included(&MIN)
    }

    fn end_bound(&self) -> Bound<&usize> {
        Bound::Included(&MAX)
    }
}

impl<const MIN: usize, const MAX: usize> Capacity for StaticMinMaxCap<MIN, MAX> {
    type Error = CompatError<Self::Min, Self::Max>;
    type FitError = FitError<Self::Min, Self::Max>;
    type Min = StaticMinCap<MIN>;
    type Max = StaticMaxCap<MAX>;

    fn min_cap(&self) -> Self::Min {
        StaticMinCap::<MIN>
    }

    fn max_cap(&self) -> Self::Max {
        StaticMaxCap::<MAX>
    }

    fn check_compatibility<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        check_static_compatibility::<Self, I>(iter)
    }

    fn check_fit<I>(&self, iter: &I) -> Result<(), Self::FitError>
    where
        I: Iterator + ?Sized,
    {
        check_static_fit::<Self, I>(iter)
    }
}

impl<const MIN: usize, const MAX: usize> From<StaticMinMaxCap<MIN, MAX>> for MinMaxCapVal {
    fn from(_value: StaticMinMaxCap<MIN, MAX>) -> Self {
        Self::new(MIN, MAX)
    }
}

/// A marker for a static exact size constraint, where `MIN == MAX`.
///
/// # Type Parameters
///
/// * `SIZE`: The size of both the inclusive minimum and maximum capacity constraints.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[debug("StaticExactCap<{SIZE}>")]
pub struct StaticExactCap<const SIZE: usize>;

impl<const SIZE: usize> StaticCap for StaticExactCap<SIZE> {
    type Cap = Self;

    const CAP: Self::Cap = Self;
}

impl<const SIZE: usize> Sealed for StaticExactCap<SIZE> {}

impl<const SIZE: usize> RangeBounds<usize> for StaticExactCap<SIZE> {
    fn start_bound(&self) -> Bound<&usize> {
        Bound::Included(&SIZE)
    }

    fn end_bound(&self) -> Bound<&usize> {
        Bound::Included(&SIZE)
    }
}

impl<const SIZE: usize> Capacity for StaticExactCap<SIZE> {
    type Error = CompatError<Self::Min, Self::Max>;
    type FitError = FitError<Self::Min, Self::Max>;
    type Min = StaticMinCap<SIZE>;
    type Max = StaticMaxCap<SIZE>;

    fn min_cap(&self) -> Self::Min {
        StaticMinCap::<SIZE>
    }

    fn max_cap(&self) -> Self::Max {
        StaticMaxCap::<SIZE>
    }

    fn check_compatibility<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        check_static_compatibility::<Self, I>(iter)
    }

    fn check_fit<I>(&self, iter: &I) -> Result<(), Self::FitError>
    where
        I: Iterator + ?Sized,
    {
        check_static_fit::<Self, I>(iter)
    }
}

impl<const SIZE: usize> From<StaticExactCap<SIZE>> for ExactCapVal {
    fn from(_value: StaticExactCap<SIZE>) -> Self {
        Self(SIZE)
    }
}

impl<const SIZE: usize> From<StaticExactCap<SIZE>> for MinMaxCapVal {
    fn from(_value: StaticExactCap<SIZE>) -> Self {
        Self::new_unchecked(SIZE, SIZE)
    }
}
