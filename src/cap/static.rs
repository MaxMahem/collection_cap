use core::convert::Infallible;
use core::ops::{Bound, RangeBounds};

use fluent_result::into::{IntoOption, IntoResult};
use tap::Pipe;

use crate::cap::{ExactCapVal, MaxCapVal, MinCapVal, MinMaxCapVal};
use crate::capacity::private::Sealed;
use crate::err::{FitBoth, FitOverflow, FitUnderflow, MaxUnderflow, MinOverflow, StaticCapError, StaticFitError};
use crate::{Capacity, ConstMaxCap, ConstMinCap, IterExt, StaticCap};

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

impl<const MIN: usize> ConstMinCap for StaticMinCap<MIN> {
    const MIN_CAP: MinCapVal = MinCapVal(MIN);
}

impl<const MIN: usize> ConstMaxCap for StaticMinCap<MIN> {
    const MAX_CAP: MaxCapVal = MaxCapVal(usize::MAX);
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

impl<const MAX: usize> ConstMaxCap for StaticMaxCap<MAX> {
    const MAX_CAP: MaxCapVal = MaxCapVal(MAX);
}

impl<const MAX: usize> Sealed for StaticMaxCap<MAX> {}

impl<const MAX: usize> ConstMinCap for StaticMaxCap<MAX> {
    const MIN_CAP: MinCapVal = MinCapVal(0);
}

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
    CAP: StaticCap<Cap = CAP> + ConstMinCap + ConstMaxCap,
    I: Iterator + ?Sized,
{
    match iter.valid_size_hint() {
        (min_size, _) if !CAP::MAX_CAP.contains(&min_size) => MinOverflow::<CAP>::new_unchecked(min_size) //
            .pipe(StaticCapError::Overflow)
            .into_err(),
        (_, Some(max_size)) if !CAP::MIN_CAP.contains(&max_size) => {
            MaxUnderflow::<CAP>::new_unchecked(max_size) //
                .pipe(StaticCapError::Underflow)
                .into_err()
        }
        _ => Ok(()),
    }
}

/// Checks if `iter` is guaranteed to fit within the static capacity constraint `C`.
fn check_static_fit<CAP, I>(iter: &I) -> Result<(), StaticFitError<CAP>>
where
    CAP: StaticCap<Cap = CAP> + ConstMinCap + ConstMaxCap,
    I: Iterator + ?Sized,
{
    let (min, max) = iter.valid_size_hint();

    let underflow = (!CAP::MIN_CAP.contains(&min)).then(|| FitUnderflow::<CAP>::new_unchecked(min));
    let overflow = match max {
        Some(max) if !CAP::MAX_CAP.contains(&max) => FitOverflow::<CAP>::fixed_unchecked(max).into_some(),
        None => Some(FitOverflow::<CAP>::UNBOUNDED),
        _ => None,
    };

    match (underflow, overflow) {
        (Some(underflow), Some(overflow)) => FitBoth::from_parts_static(overflow, underflow) //
            .pipe(StaticFitError::Both)
            .into_err(),
        (Some(underflow), None) => StaticFitError::Underflow(underflow).into_err(),
        (None, Some(overflow)) => StaticFitError::Overflow(overflow).into_err(),
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

impl<const MIN: usize, const MAX: usize> ConstMinCap for StaticMinMaxCap<MIN, MAX> {
    const MIN_CAP: MinCapVal = {
        let () = Self::_CHECK;
        MinCapVal(MIN)
    };
}

impl<const MIN: usize, const MAX: usize> ConstMaxCap for StaticMinMaxCap<MIN, MAX> {
    const MAX_CAP: MaxCapVal = {
        let () = Self::_CHECK;
        MaxCapVal(MAX)
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

impl<const SIZE: usize> ConstMinCap for StaticExactCap<SIZE> {
    const MIN_CAP: MinCapVal = MinCapVal(SIZE);
}

impl<const SIZE: usize> ConstMaxCap for StaticExactCap<SIZE> {
    const MAX_CAP: MaxCapVal = MaxCapVal(SIZE);
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
