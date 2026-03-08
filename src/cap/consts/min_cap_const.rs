use core::ops::{Bound, RangeBounds, RangeFrom};

use derive_more::Debug;
use fluent_result::into::IntoResult;

use crate::cap::{MinCapVal, UnboundedCap};
use crate::err::{MaxUnderflow, MinUnderflow};
use crate::internal::{Ok, Sealed};
use crate::{Capacity, ConstCap, IterExt};

/// A `const` minimum [`Capacity`] constraint.
///
/// # Type Parameters
///
/// * `MIN`: The inclusive minimum size of the [`Capacity`] constraint.
#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
#[debug("ConstMinCap<{MIN}>")]
pub struct ConstMinCap<const MIN: usize>;

impl<const MIN: usize> ConstMinCap<MIN> {
    /// The equivalent range.
    pub const RANGE: RangeFrom<usize> = MIN..;
}

impl<const MIN: usize> Capacity for ConstMinCap<MIN> {
    type IntersectError = MaxUnderflow<Self>;
    type OverlapError = MinUnderflow<Self>;
    type Min = Self;
    type Max = UnboundedCap;

    fn min_cap(&self) -> Self::Min {
        *self
    }

    fn max_cap(&self) -> Self::Max {
        UnboundedCap
    }

    fn contains_size(&self, size: usize) -> bool {
        size >= MIN
    }

    fn check_intersects<I>(&self, iter: &I) -> Result<(), Self::IntersectError>
    where
        I: Iterator + ?Sized,
    {
        match iter.valid_size_hint() {
            (_, Some(max)) if !self.contains_size(max) // fmt
                => MaxUnderflow::from_parts(max, Self).into_err(),
            _ => Ok!(),
        }
    }

    fn check_overlaps<I>(&self, iter: &I) -> Result<(), Self::OverlapError>
    where
        I: Iterator + ?Sized,
    {
        match iter.valid_size_hint() {
            (min, _) if !self.contains_size(min) // fmt
                => MinUnderflow::from_parts(min, Self).into_err(),
            _ => Ok!(),
        }
    }
}

impl<const MIN: usize> ConstCap for ConstMinCap<MIN> {
    type Cap = Self;

    const CAP: Self::Cap = Self;
}

impl<const MIN: usize> RangeBounds<usize> for ConstMinCap<MIN> {
    fn start_bound(&self) -> Bound<&usize> {
        Bound::Included(&MIN)
    }

    fn end_bound(&self) -> Bound<&usize> {
        Bound::Unbounded
    }
}

impl<const MIN: usize> From<ConstMinCap<MIN>> for MinCapVal {
    fn from(_value: ConstMinCap<MIN>) -> Self {
        Self(MIN)
    }
}

impl<const MIN: usize> From<ConstMinCap<MIN>> for RangeFrom<usize> {
    fn from(_value: ConstMinCap<MIN>) -> Self {
        MIN..
    }
}

impl<const MIN: usize> Sealed for ConstMinCap<MIN> {}
