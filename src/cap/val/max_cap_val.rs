use core::ops::{Bound, RangeBounds, RangeTo, RangeToInclusive};

use derive_more::{From, Into};
use fluent_result::into::IntoResult;

use crate::cap::UnboundedCap;
use crate::err::{EmptyRange, MaxOverflow, MinOverflow};
use crate::internal::Ok;
use crate::{Capacity, IterExt};

/// A runtime constraint specifying a maximum [`Capacity`].
#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, From, Into)]
pub struct MaxCapVal(pub usize);

impl MaxCapVal {
    /// A [`Capacity`] constraint representing a maximum of zero elements.
    pub const ZERO: Self = Self(0);
}

impl Capacity for MaxCapVal {
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
        size <= self.0
    }

    fn check_intersects<I>(&self, iter: &I) -> Result<(), Self::IntersectError>
    where
        I: Iterator + ?Sized,
    {
        match iter.valid_size_hint() {
            (min_size, _) if !self.contains_size(min_size) // fmt
                => MinOverflow::from_parts(min_size, *self).into_err(),
            _ => Ok!(),
        }
    }

    fn check_overlaps<I>(&self, iter: &I) -> Result<(), Self::OverlapError>
    where
        I: Iterator + ?Sized,
    {
        match iter.valid_size_hint() {
            (_, Some(max)) if !self.contains_size(max) // fmt
                => MaxOverflow::from_parts_fixed(max, *self).into_err(),
            (_, None) => MaxOverflow::unbounded(*self).into_err(),
            _ => Ok!(),
        }
    }
}

crate::cap::val::impl_variable_cap_from_self!(MaxCapVal);

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

crate::internal::impl_sealed!(MaxCapVal);
