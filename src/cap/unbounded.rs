use core::convert::Infallible;
use core::ops::{Bound, RangeBounds};
use derive_more::{Display, From, Into};

use crate::internal::{Ok, Sealed};
use crate::{Capacity, StaticCap};

/// A runtime constraint specifying an unbounded capacity.
///
/// This constraint is compatible with any iterator.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, From, Into, Display)]
pub struct UnboundedCap;

impl Sealed for UnboundedCap {}

impl Capacity for UnboundedCap {
    type CapError = Infallible;
    type FitError = Infallible;
    type Min = Self;
    type Max = Self;

    fn min_cap(&self) -> Self::Min {
        *self
    }

    fn max_cap(&self) -> Self::Max {
        *self
    }

    /// Always returns `true` as an unbounded capacity constraint is
    /// compatible with any size.
    fn contains_size(&self, _size: usize) -> bool {
        true
    }

    /// Always returns `Ok(())` as an unbounded capacity constraint is
    /// compatible with any iterator.
    fn check_compatibility<I>(&self, _iter: &I) -> Result<(), Self::CapError>
    where
        I: Iterator + ?Sized,
    {
        Ok!()
    }

    /// Always returns `Ok(())` as an unbounded capacity constraint fits
    /// any iterator.
    fn check_fit<I>(&self, _iter: &I) -> Result<(), Self::FitError>
    where
        I: Iterator + ?Sized,
    {
        Ok!()
    }
}

crate::cap::val::impl_variable_cap_from_self!(UnboundedCap);

impl StaticCap for UnboundedCap {
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
