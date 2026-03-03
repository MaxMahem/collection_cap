use core::convert::Infallible;
use core::ops::{Bound, RangeBounds};

use fluent_result::into::{IntoOption, IntoResult};
use tap::Pipe;

use crate::cap::UnboundedCapVal;
use crate::cap::{ExactCapVal, MaxCapVal, MinCapVal, MinMaxCapVal};
use crate::capacity::private::Sealed;
use crate::err::{
    CompatError, FitBoth, FitError, FitOverflow, FitUnderflow, MaxUnderflow, MinOverflow, StaticCapError,
    StaticFitError,
};
use crate::{Capacity, IterExt, StaticCap};

/// A static minimum capacity constraint.
///
/// # Type Parameters
///
/// * `MIN`: The inclusive minimum size of the constraint.
#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
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
    type FitError = FitUnderflow<Self>;
    type Min = Self;
    type Max = StaticUnboundedCap;

    fn min_cap(&self) -> Self::Min {
        *self
    }

    fn max_cap(&self) -> Self::Max {
        StaticUnboundedCap
    }

    fn check_compatibility<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        match iter.valid_size_hint() {
            (_, Some(max)) if !self.contains(&max) => MaxUnderflow::new_unchecked(max).into_err(),
            _ => Ok(()),
        }
    }

    fn check_fit<I>(&self, iter: &I) -> Result<(), Self::FitError>
    where
        I: Iterator + ?Sized,
    {
        match iter.valid_size_hint() {
            (min, _) if !self.contains(&min) => FitUnderflow::new_unchecked(min).into_err(),
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
    type FitError = FitOverflow<Self>;
    type Min = StaticUnboundedCap;
    type Max = Self;

    fn min_cap(&self) -> Self::Min {
        StaticUnboundedCap
    }

    fn max_cap(&self) -> Self::Max {
        *self
    }

    fn check_compatibility<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        match iter.valid_size_hint() {
            (min_size, _) if !self.contains(&min_size) => MinOverflow::new_unchecked(min_size).into_err(),
            _ => Ok(()),
        }
    }

    fn check_fit<I>(&self, iter: &I) -> Result<(), Self::FitError>
    where
        I: Iterator + ?Sized,
    {
        match iter.valid_size_hint() {
            (_, Some(max)) if !self.contains(&max) => FitOverflow::<Self>::fixed_unchecked(max).into_err(),
            (_, None) => Err(FitOverflow::<Self>::UNBOUNDED),
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
fn check_static_compatibility<CAP, I>(iter: &I) -> Result<(), StaticCapError<CAP>>
where
    CAP: StaticCap<Cap = CAP> + Capacity,
    I: Iterator + ?Sized,
{
    match iter.valid_size_hint() {
        (min_size, _) if !CAP::CAP.max_cap().contains(&min_size) => {
            MinOverflow::<<CAP as Capacity>::Max>::from_parts(min_size, CAP::CAP.max_cap()) //
                .pipe(CompatError::Overflow)
                .into_err()
        }
        (_, Some(max_size)) if !CAP::CAP.min_cap().contains(&max_size) => {
            MaxUnderflow::<<CAP as Capacity>::Min>::from_parts(max_size, CAP::CAP.min_cap()) //
                .pipe(CompatError::Underflow)
                .into_err()
        }
        _ => Ok(()),
    }
}

/// Checks if `iter` is guaranteed to fit within the static capacity constraint `C`.
fn check_static_fit<CAP, I>(iter: &I) -> Result<(), StaticFitError<CAP>>
where
    CAP: StaticCap<Cap = CAP> + Capacity,
    I: Iterator + ?Sized,
{
    let (min, max) = iter.valid_size_hint();

    let underflow = (!CAP::CAP.min_cap().contains(&min))
        .then(|| FitUnderflow::<<CAP as Capacity>::Min>::from_parts(min, CAP::CAP.min_cap()));
    let overflow = match max {
        Some(max) if !CAP::CAP.max_cap().contains(&max) => {
            FitOverflow::<<CAP as Capacity>::Max>::from_parts(max, CAP::CAP.max_cap()).into_some()
        }
        None => Some(FitOverflow::<<CAP as Capacity>::Max>::unbounded_unchecked(CAP::CAP.max_cap())),
        _ => None,
    };

    match (underflow, overflow) {
        (Some(underflow), Some(overflow)) => FitBoth::from_parts(overflow, underflow) //
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
pub struct StaticMinMaxCap<const MIN: usize, const MAX: usize>;

impl<const MIN: usize, const MAX: usize> StaticMinMaxCap<MIN, MAX> {
    const _CHECK: () = {
        assert!(MIN <= MAX, "StaticMinMaxCap: MIN must be <= MAX");
    };
}

impl<const MIN: usize, const MAX: usize> StaticCap for StaticMinMaxCap<MIN, MAX> {
    type Cap = Self;

    const CAP: Self::Cap = {
        let () = Self::_CHECK;
        Self
    };
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
    type Error = StaticCapError<Self>;
    type FitError = StaticFitError<Self>;
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
    type Error = StaticCapError<Self>;
    type FitError = StaticFitError<Self>;
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
        Self::new(SIZE, SIZE)
    }
}

/// A static unbounded capacity constraint.
///
/// This constraint is compatible with any iterator.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct StaticUnboundedCap;

impl StaticCap for StaticUnboundedCap {
    type Cap = Self;

    const CAP: Self::Cap = Self;
}

impl Sealed for StaticUnboundedCap {}

impl RangeBounds<usize> for StaticUnboundedCap {
    fn start_bound(&self) -> Bound<&usize> {
        Bound::Unbounded
    }

    fn end_bound(&self) -> Bound<&usize> {
        Bound::Unbounded
    }
}

impl Capacity for StaticUnboundedCap {
    type Error = Infallible;
    type FitError = Infallible;
    type Min = UnboundedCapVal;
    type Max = UnboundedCapVal;

    fn min_cap(&self) -> Self::Min {
        UnboundedCapVal
    }

    fn max_cap(&self) -> Self::Max {
        UnboundedCapVal
    }

    fn check_compatibility<I>(&self, _iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        Ok(())
    }

    /// Always returns `Ok(())` as an unbounded capacity constraint fits
    /// any iterator.
    fn check_fit<I>(&self, _iter: &I) -> Result<(), Self::FitError>
    where
        I: Iterator + ?Sized,
    {
        Ok(())
    }
}
