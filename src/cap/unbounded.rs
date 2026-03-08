use core::convert::Infallible;
use core::ops::{Bound, RangeBounds};
use derive_more::{Display, From, Into};

use crate::internal::{Ok, Sealed};
use crate::{Capacity, ConstCap};

/// A runtime constraint specifying an unbounded [`Capacity`].
///
/// This constraint intersects with any [`Iterator`].
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, From, Into, Display)]
pub struct UnboundedCap;

impl Sealed for UnboundedCap {}

impl Capacity for UnboundedCap {
    type IntersectError = Infallible;
    type OverlapError = Infallible;
    type Min = Self;
    type Max = Self;

    fn min_cap(&self) -> Self::Min {
        *self
    }

    fn max_cap(&self) -> Self::Max {
        *self
    }

    /// Always returns `true` as an unbounded [`Capacity`] constraint is
    /// intersecting with any size.
    fn contains_size(&self, _size: usize) -> bool {
        true
    }

    /// Always returns `Ok(())` as an unbounded [`Capacity`] constraint is
    /// intersecting with any [`Iterator`].
    fn check_intersects<I>(&self, _iter: &I) -> Result<(), Self::IntersectError>
    where
        I: Iterator + ?Sized,
    {
        Ok!()
    }

    /// Always returns `Ok(())` as an unbounded [`Capacity`] constraint overlaps
    /// any [`Iterator`].
    fn check_overlaps<I>(&self, _iter: &I) -> Result<(), Self::OverlapError>
    where
        I: Iterator + ?Sized,
    {
        Ok!()
    }
}

crate::cap::val::impl_variable_cap_from_self!(UnboundedCap);

impl ConstCap for UnboundedCap {
    type Cap = Self;

    const CAP: Self::Cap = Self;
}

impl RangeBounds<usize> for UnboundedCap {
    fn start_bound(&self) -> Bound<&usize> {
        Bound::Unbounded
    }
    fn end_bound(&self) -> Bound<&usize> {
        Bound::Unbounded
    }
}
