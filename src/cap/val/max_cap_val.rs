use core::ops::{Bound, RangeBounds, RangeTo, RangeToInclusive};

use crate::cap::UnboundedCap;
use crate::err::{EmptyRange, MaxOverflow, MinOverflow};
use crate::internal::{Ok, Sealed};
use crate::{Capacity, IterExt, VariableCap};
use derive_more::{From, Into};
use fluent_result::into::IntoResult;

/// A runtime constraint specifying a maximum capacity.
#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, From, Into)]
pub struct MaxCapVal(pub usize);

impl MaxCapVal {
    /// A capacity constraint representing a maximum of zero elements.
    pub const ZERO: Self = Self(0);
}

impl Sealed for MaxCapVal {}

impl Capacity for MaxCapVal {
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
                => MinOverflow::from_parts(min_size, *self).into_err(),
            _ => Ok!(),
        }
    }

    fn check_fit<I>(&self, iter: &I) -> Result<(), Self::FitError>
    where
        I: Iterator + ?Sized,
    {
        match iter.valid_size_hint() {
            (_, Some(max)) if !self.contains(&max) // fmt
                => MaxOverflow::from_parts(max, *self).into_err(),
            (_, None) => MaxOverflow::unbounded(*self).into_err(),
            _ => Ok!(),
        }
    }
}

impl VariableCap for MaxCapVal {
    type Cap = Self;

    fn capacity(&self) -> Self {
        *self
    }
}

impl TryFrom<RangeTo<usize>> for MaxCapVal {
    type Error = EmptyRange;
    fn try_from(value: RangeTo<usize>) -> Result<Self, Self::Error> {
        usize::checked_sub(value.end, 1).map(Self).ok_or(EmptyRange)
    }
}

impl From<RangeToInclusive<usize>> for MaxCapVal {
    fn from(value: RangeToInclusive<usize>) -> Self {
        Self(value.end)
    }
}

impl RangeBounds<usize> for MaxCapVal {
    fn start_bound(&self) -> Bound<&usize> {
        Bound::Unbounded
    }
    fn end_bound(&self) -> Bound<&usize> {
        Bound::Included(&self.0)
    }
}
