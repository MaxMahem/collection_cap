use core::ops::{Bound, RangeBounds, RangeInclusive};

use derive_more::Debug;

use crate::cap::{ConstMaxCap, ConstMinCap, ExactCapVal, MinMaxCapVal};
use crate::err::{IntersectError, OverlapError};
use crate::internal::Sealed;
use crate::{Capacity, ConstCap};

/// A `const` exact size [`Capacity`] constraint, where `MIN == MAX`.
///
/// # Type Parameters
///
/// * `SIZE`: The size of both the inclusive minimum and maximum [`Capacity`] constraints.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[debug("ConstExactCap<{SIZE}>")]
pub struct ConstExactCap<const SIZE: usize>;

impl<const SIZE: usize> ConstExactCap<SIZE> {
    /// The equivalent range.
    pub const RANGE: RangeInclusive<usize> = SIZE..=SIZE;
}

impl<const SIZE: usize> Capacity for ConstExactCap<SIZE> {
    type IntersectError = IntersectError<Self::Min, Self::Max>;
    type OverlapError = OverlapError<Self::Min, Self::Max>;
    type Min = ConstMinCap<SIZE>;
    type Max = ConstMaxCap<SIZE>;

    fn min_cap(&self) -> Self::Min {
        ConstMinCap::<SIZE>
    }

    fn max_cap(&self) -> Self::Max {
        ConstMaxCap::<SIZE>
    }

    fn contains_size(&self, size: usize) -> bool {
        size == SIZE
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

impl<const SIZE: usize> ConstCap for ConstExactCap<SIZE> {
    type Cap = Self;

    const CAP: Self::Cap = Self;
}

impl<const SIZE: usize> RangeBounds<usize> for ConstExactCap<SIZE> {
    fn start_bound(&self) -> Bound<&usize> {
        Bound::Included(&SIZE)
    }

    fn end_bound(&self) -> Bound<&usize> {
        Bound::Included(&SIZE)
    }
}

impl<const SIZE: usize> From<ConstExactCap<SIZE>> for ExactCapVal {
    fn from(_value: ConstExactCap<SIZE>) -> Self {
        Self(SIZE)
    }
}

impl<const SIZE: usize> From<ConstExactCap<SIZE>> for MinMaxCapVal {
    fn from(_value: ConstExactCap<SIZE>) -> Self {
        Self::new_unchecked(SIZE, SIZE)
    }
}

impl<const SIZE: usize> From<ConstExactCap<SIZE>> for RangeInclusive<usize> {
    fn from(_value: ConstExactCap<SIZE>) -> Self {
        SIZE..=SIZE
    }
}

impl<const SIZE: usize> Sealed for ConstExactCap<SIZE> {}
