use core::ops::{Bound, RangeBounds, RangeToInclusive};

use derive_more::Debug;
use fluent_result::into::IntoResult;

use crate::cap::{MaxCapVal, UnboundedCap};
use crate::err::{MaxOverflow, MinOverflow};
use crate::internal::{Ok, Sealed};
use crate::{Capacity, ConstCap, IterExt};

/// A `const` maximum [`Capacity`] constraint.
///
/// # Type Parameters
///
/// * `MAX`: The inclusive maximum size of the [`Capacity`] constraint.
#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
#[debug("ConstMaxCap<{MAX}>")]
pub struct ConstMaxCap<const MAX: usize>;

impl<const MAX: usize> ConstMaxCap<MAX> {
    /// The equivalent range.
    pub const RANGE: RangeToInclusive<usize> = ..=MAX;
}

impl<const MAX: usize> Capacity for ConstMaxCap<MAX> {
    type IntersectError = MinOverflow<Self>;
    type OverlapError = MaxOverflow<Self>;
    type Min = UnboundedCap;
    type Max = Self;

    fn min_cap(&self) -> Self::Min {
        UnboundedCap
    }

    fn max_cap(&self) -> Self::Max {
        *self
    }

    fn contains_size(&self, size: usize) -> bool {
        size <= MAX
    }

    fn check_intersects<I>(&self, iter: &I) -> Result<(), Self::IntersectError>
    where
        I: Iterator + ?Sized,
    {
        match iter.valid_size_hint() {
            (min_size, _) if !self.contains_size(min_size) // fmt
                => MinOverflow::from_parts(min_size, Self).into_err(),
            _ => Ok!(),
        }
    }

    fn check_overlaps<I>(&self, iter: &I) -> Result<(), Self::OverlapError>
    where
        I: Iterator + ?Sized,
    {
        match iter.valid_size_hint() {
            (_, Some(max)) if !self.contains_size(max) // fmt
                => MaxOverflow::from_parts_fixed(max, Self).into_err(),
            (_, None) => Err(MaxOverflow::<Self>::UNBOUNDED),
            _ => Ok!(),
        }
    }
}

impl<const MAX: usize> ConstCap for ConstMaxCap<MAX> {
    type Cap = Self;

    const CAP: Self::Cap = Self;
}

impl<const MAX: usize> RangeBounds<usize> for ConstMaxCap<MAX> {
    fn start_bound(&self) -> Bound<&usize> {
        Bound::Unbounded
    }

    fn end_bound(&self) -> Bound<&usize> {
        Bound::Included(&MAX)
    }
}

impl<const MAX: usize> From<ConstMaxCap<MAX>> for MaxCapVal {
    fn from(_value: ConstMaxCap<MAX>) -> Self {
        Self(MAX)
    }
}

impl<const MAX: usize> From<ConstMaxCap<MAX>> for RangeToInclusive<usize> {
    fn from(_value: ConstMaxCap<MAX>) -> Self {
        ..=MAX
    }
}

impl<const MAX: usize> Sealed for ConstMaxCap<MAX> {}
