use core::ops::{Bound, RangeBounds, RangeInclusive};
use derive_more::Debug;

use crate::cap::{MinMaxCapVal, StaticMaxCap, StaticMinCap};
use crate::err::{CompatError, FitError};
use crate::internal::{Sealed, assert_then};
use crate::{Capacity, StaticCap};

use super::{check_static_compatibility, check_static_fit};

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

impl<const MIN: usize, const MAX: usize> StaticMinMaxCap<MIN, MAX> {
    /// The equivalent range.
    pub const RANGE: RangeInclusive<usize> = MIN..=MAX;
}

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
    type CapError = CompatError<Self::Min, Self::Max>;
    type FitError = FitError<Self::Min, Self::Max>;
    type Min = StaticMinCap<MIN>;
    type Max = StaticMaxCap<MAX>;

    fn min_cap(&self) -> Self::Min {
        StaticMinCap::<MIN>
    }

    fn max_cap(&self) -> Self::Max {
        StaticMaxCap::<MAX>
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

impl<const MIN: usize, const MAX: usize> From<StaticMinMaxCap<MIN, MAX>> for MinMaxCapVal {
    fn from(_value: StaticMinMaxCap<MIN, MAX>) -> Self {
        Self::new(MIN, MAX)
    }
}

impl<const MIN: usize, const MAX: usize> From<StaticMinMaxCap<MIN, MAX>> for RangeInclusive<usize> {
    fn from(_value: StaticMinMaxCap<MIN, MAX>) -> Self {
        MIN..=MAX
    }
}
