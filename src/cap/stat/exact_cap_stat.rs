use core::ops::{Bound, RangeBounds, RangeInclusive};

use derive_more::Debug;

use crate::cap::{ExactCapVal, MinMaxCapVal, StaticMaxCap, StaticMinCap};
use crate::err::{CompatError, FitError};
use crate::internal::Sealed;
use crate::{Capacity, StaticCap};

use super::{check_static_compatibility, check_static_fit};

/// A marker for a static exact size constraint, where `MIN == MAX`.
///
/// # Type Parameters
///
/// * `SIZE`: The size of both the inclusive minimum and maximum capacity constraints.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[debug("StaticExactCap<{SIZE}>")]
pub struct StaticExactCap<const SIZE: usize>;

impl<const SIZE: usize> StaticExactCap<SIZE> {
    /// The equivalent range.
    pub const RANGE: RangeInclusive<usize> = SIZE..=SIZE;
}

impl<const SIZE: usize> Capacity for StaticExactCap<SIZE> {
    type CapError = CompatError<Self::Min, Self::Max>;
    type FitError = FitError<Self::Min, Self::Max>;
    type Min = StaticMinCap<SIZE>;
    type Max = StaticMaxCap<SIZE>;

    fn min_cap(&self) -> Self::Min {
        StaticMinCap::<SIZE>
    }

    fn max_cap(&self) -> Self::Max {
        StaticMaxCap::<SIZE>
    }

    fn check_compatibility<I>(&self, iter: &I) -> Result<(), Self::CapError>
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

impl<const SIZE: usize> StaticCap for StaticExactCap<SIZE> {
    type Cap = Self;

    const CAP: Self::Cap = Self;
}

impl<const SIZE: usize> RangeBounds<usize> for StaticExactCap<SIZE> {
    fn start_bound(&self) -> Bound<&usize> {
        Bound::Included(&SIZE)
    }

    fn end_bound(&self) -> Bound<&usize> {
        Bound::Included(&SIZE)
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

impl<const SIZE: usize> From<StaticExactCap<SIZE>> for RangeInclusive<usize> {
    fn from(_value: StaticExactCap<SIZE>) -> Self {
        SIZE..=SIZE
    }
}

impl<const SIZE: usize> Sealed for StaticExactCap<SIZE> {}
