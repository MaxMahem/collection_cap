use core::ops::{Bound, RangeBounds, RangeToInclusive};
use derive_more::Debug;

use fluent_result::into::IntoResult;

use crate::cap::{MaxCapVal, UnboundedCap};
use crate::err::{MaxOverflow, MinOverflow};
use crate::internal::{Ok, Sealed};
use crate::{Capacity, IterExt, StaticCap};

/// A static maximum capacity constraint.
///
/// # Type Parameters
///
/// * `MAX`: The inclusive maximum size of the constraint.
#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
#[debug("StaticMaxCap<{MAX}>")]
pub struct StaticMaxCap<const MAX: usize>;

impl<const MAX: usize> StaticMaxCap<MAX> {
    /// The equivalent range.
    pub const RANGE: RangeToInclusive<usize> = ..=MAX;
}

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
    type CapError = MinOverflow<Self>;
    type FitError = MaxOverflow<Self>;
    type Min = UnboundedCap;
    type Max = Self;

    fn min_cap(&self) -> Self::Min {
        UnboundedCap
    }

    fn max_cap(&self) -> Self::Max {
        *self
    }

    fn check_compatibility<I>(&self, iter: &I) -> Result<(), Self::CapError>
    where
        I: Iterator + ?Sized,
    {
        match iter.valid_size_hint() {
            (min_size, _) if !self.contains(&min_size) // fmt
                => MinOverflow::from_parts(min_size, Self).into_err(),
            _ => Ok!(),
        }
    }

    fn check_fit<I>(&self, iter: &I) -> Result<(), Self::FitError>
    where
        I: Iterator + ?Sized,
    {
        match iter.valid_size_hint() {
            (_, Some(max)) if !self.contains(&max) // fmt
                => MaxOverflow::from_parts(max, Self).into_err(),
            (_, None) => Err(MaxOverflow::<Self>::UNBOUNDED),
            _ => Ok!(),
        }
    }
}

impl<const MAX: usize> From<StaticMaxCap<MAX>> for MaxCapVal {
    fn from(_value: StaticMaxCap<MAX>) -> Self {
        Self(MAX)
    }
}

impl<const MAX: usize> From<StaticMaxCap<MAX>> for RangeToInclusive<usize> {
    fn from(_value: StaticMaxCap<MAX>) -> Self {
        ..=MAX
    }
}
