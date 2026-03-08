use core::ops::{Bound, RangeBounds, RangeInclusive};

use derive_more::Debug;

use crate::cap::{ConstMaxCap, ConstMinCap, MinMaxCapVal};
use crate::err::{IntersectError, OverlapError};
use crate::internal::{Sealed, assert_then};
use crate::{Capacity, ConstCap};

/// A `const` minimum and maximum [`Capacity`] constraint.
///
/// If `MIN == MAX`, then consider using
/// [`ConstExactCap`](crate::cap::ConstExactCap) instead.
///
/// # Type Parameters
///
/// * `MIN`: The inclusive minimum size of the [`Capacity`] constraint.
/// * `MAX`: The inclusive maximum size of the constraint.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[debug("ConstMinMaxCap<{MIN}, {MAX}>")]
pub struct ConstMinMaxCap<const MIN: usize, const MAX: usize>;

impl<const MIN: usize, const MAX: usize> ConstMinMaxCap<MIN, MAX> {
    /// The equivalent range.
    pub const RANGE: RangeInclusive<usize> =
        assert_then!(MIN <= MAX => MIN..=MAX, "ConstMinMaxCap: MIN must be <= MAX");
}

impl<const MIN: usize, const MAX: usize> Capacity for ConstMinMaxCap<MIN, MAX> {
    type IntersectError = IntersectError<Self::Min, Self::Max>;
    type OverlapError = OverlapError<Self::Min, Self::Max>;
    type Min = ConstMinCap<MIN>;
    type Max = ConstMaxCap<MAX>;

    fn min_cap(&self) -> Self::Min {
        ConstMinCap::<MIN>
    }

    fn max_cap(&self) -> Self::Max {
        ConstMaxCap::<MAX>
    }

    fn contains_size(&self, size: usize) -> bool {
        size >= MIN && size <= MAX
    }

    fn check_intersects<I>(&self, iter: &I) -> Result<(), Self::IntersectError>
    where
        I: Iterator + ?Sized,
    {
        super::check_const::check_const_intersect(iter)
    }

    fn check_overlaps<I>(&self, iter: &I) -> Result<(), Self::OverlapError>
    where
        I: Iterator + ?Sized,
    {
        super::check_const::check_const_overlaps(iter)
    }
}

impl<const MIN: usize, const MAX: usize> ConstCap for ConstMinMaxCap<MIN, MAX> {
    type Cap = Self;

    const CAP: Self::Cap = assert_then!(MIN <= MAX => Self, "ConstMinMaxCap: MIN must be <= MAX");
}

impl<const MIN: usize, const MAX: usize> RangeBounds<usize> for ConstMinMaxCap<MIN, MAX> {
    fn start_bound(&self) -> Bound<&usize> {
        Bound::Included(&MIN)
    }

    fn end_bound(&self) -> Bound<&usize> {
        Bound::Included(&MAX)
    }
}

impl<const MIN: usize, const MAX: usize> From<ConstMinMaxCap<MIN, MAX>> for MinMaxCapVal {
    fn from(_value: ConstMinMaxCap<MIN, MAX>) -> Self {
        Self::new(MIN, MAX)
    }
}

impl<const MIN: usize, const MAX: usize> From<ConstMinMaxCap<MIN, MAX>> for RangeInclusive<usize> {
    fn from(_value: ConstMinMaxCap<MIN, MAX>) -> Self {
        MIN..=MAX
    }
}

impl<const MIN: usize, const MAX: usize> Sealed for ConstMinMaxCap<MIN, MAX> {}
