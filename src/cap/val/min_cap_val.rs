use core::ops::{Bound, RangeBounds, RangeFrom};

use crate::cap::UnboundedCap;
use crate::err::{MaxUnderflow, MinUnderflow};
use crate::internal::{Ok, Sealed};
use crate::{Capacity, IterExt, VariableCap};
use derive_more::{From, Into};
use fluent_result::into::IntoResult;

/// A runtime constraint specifying a minimum capacity.
#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, From, Into)]
pub struct MinCapVal(pub usize);

impl Sealed for MinCapVal {}

impl Capacity for MinCapVal {
    type CapError = MaxUnderflow<Self>;
    type FitError = MinUnderflow<Self>;
    type Min = Self;
    type Max = UnboundedCap;

    fn min_cap(&self) -> Self::Min {
        *self
    }

    fn max_cap(&self) -> Self::Max {
        UnboundedCap
    }

    fn check_compatibility<I>(&self, iter: &I) -> Result<(), Self::CapError>
    where
        I: Iterator + ?Sized,
    {
        match iter.valid_size_hint() {
            (_, Some(max_size)) if !self.contains(&max_size) // fmt
                => MaxUnderflow::from_parts(max_size, *self).into_err(),
            _ => Ok!(),
        }
    }

    fn check_fit<I>(&self, iter: &I) -> Result<(), Self::FitError>
    where
        I: Iterator + ?Sized,
    {
        match iter.valid_size_hint() {
            (min, _) if !self.contains(&min) // fmt
                => MinUnderflow::from_parts(min, *self).into_err(),
            _ => Ok!(),
        }
    }
}

impl VariableCap for MinCapVal {
    type Cap = Self;

    fn capacity(&self) -> Self {
        *self
    }
}

impl From<RangeFrom<usize>> for MinCapVal {
    fn from(value: RangeFrom<usize>) -> Self {
        Self(value.start)
    }
}

impl RangeBounds<usize> for MinCapVal {
    fn start_bound(&self) -> Bound<&usize> {
        Bound::Included(&self.0)
    }
    fn end_bound(&self) -> Bound<&usize> {
        Bound::Unbounded
    }
}
