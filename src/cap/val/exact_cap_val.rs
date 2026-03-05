use core::ops::{Bound, RangeBounds};

use derive_more::{From, Into};

use crate::Capacity;
use crate::cap::val::{MaxCapVal, MinCapVal, MinMaxCapVal};
use crate::err::{CompatError, FitError};

/// A runtime constraint specifying an exact capacity.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, From, Into)]
pub struct ExactCapVal(pub usize);

impl ExactCapVal {
    /// A capacity constraint representing exactly zero elements.
    pub const ZERO: Self = Self(0);
}

impl Capacity for ExactCapVal {
    type CapError = CompatError<MinCapVal, MaxCapVal>;
    type FitError = FitError<MinCapVal, MaxCapVal>;
    type Min = MinCapVal;
    type Max = MaxCapVal;

    fn min_cap(&self) -> Self::Min {
        MinCapVal(self.0)
    }

    fn max_cap(&self) -> Self::Max {
        MaxCapVal(self.0)
    }

    fn check_compatibility<I>(&self, iter: &I) -> Result<(), Self::CapError>
    where
        I: Iterator + ?Sized,
    {
        MinMaxCapVal::from(*self).check_compatibility(iter)
    }

    fn check_fit<I>(&self, iter: &I) -> Result<(), Self::FitError>
    where
        I: Iterator + ?Sized,
    {
        MinMaxCapVal::from(*self).check_fit(iter)
    }
}

crate::cap::val::impl_variable_cap!(ExactCapVal);

impl RangeBounds<usize> for ExactCapVal {
    fn start_bound(&self) -> Bound<&usize> {
        Bound::Included(&self.0)
    }
    fn end_bound(&self) -> Bound<&usize> {
        Bound::Included(&self.0)
    }
}

impl PartialEq<MinMaxCapVal> for ExactCapVal {
    fn eq(&self, other: &MinMaxCapVal) -> bool {
        self.0 == other.min().0 && self.0 == other.max().0
    }
}

crate::internal::impl_sealed!(ExactCapVal);
