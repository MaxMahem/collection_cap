use core::ops::{Bound, RangeBounds};

use derive_more::{Display, From, Into};
use fluent_result::into::IntoResult;
use size_hinter::{InvalidSizeHint, SizeHint};
use tap::TryConv;

use crate::Capacity;
use crate::INVALID_SIZE_HINT_MSG;
use crate::err::{CapError, Overflows, Underflows};

/// A runtime constraint specifying a maximum capacity.
#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, From, Into, Display)]
pub struct MaxCapVal(pub usize);

impl Capacity for MaxCapVal {
    type Error = Overflows;

    fn check_compatability<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        match iter.size_hint().try_conv::<SizeHint>().map(Into::into) {
            Ok((min_size, _)) if !self.contains(&min_size) => Overflows::from_cap(min_size, *self).into_err(),
            Ok(_) => Ok(()),
            Err(InvalidSizeHint) => panic!("{INVALID_SIZE_HINT_MSG}"),
        }
    }
}

impl From<ExactCapVal> for MaxCapVal {
    fn from(value: ExactCapVal) -> Self {
        Self(value.0)
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

/// A runtime constraint specifying a minimum capacity.
#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, From, Into, Display)]
pub struct MinCapVal(pub usize);

impl Capacity for MinCapVal {
    type Error = Underflows;

    fn check_compatability<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        match iter.size_hint().try_conv::<SizeHint>().map(Into::into) {
            Ok((_, Some(max_size))) if !self.contains(&max_size) => Underflows::from_cap(max_size, *self).into_err(),
            Ok(_) => Ok(()),
            Err(InvalidSizeHint) => panic!("{INVALID_SIZE_HINT_MSG}"),
        }
    }
}

impl From<ExactCapVal> for MinCapVal {
    fn from(value: ExactCapVal) -> Self {
        Self(value.0)
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

/// A runtime constraint specifying both a minimum and maximum capacity.
#[derive(Debug, PartialEq, Eq, Copy, Clone, derive_more::Into)]
pub struct MinMaxCapVal {
    /// The minimum capacity required.
    min: MinCapVal,
    /// The maximum capacity allowed.
    max: MaxCapVal,
}

impl MinMaxCapVal {
    /// Creates a new [`MinMaxCapVal`].
    ///
    /// # Panics
    ///
    /// Panics if `min > max`.
    #[must_use]
    pub const fn new(min: usize, max: usize) -> Self {
        match (min, max) {
            (min, max) if min > max => panic!("Invalid range (start > end)"),
            (min, max) => Self { min: MinCapVal(min), max: MaxCapVal(max) },
        }
    }

    /// Returns the minimum capacity required.
    #[must_use]
    pub const fn min(&self) -> MinCapVal {
        self.min
    }

    /// Returns the maximum capacity allowed.
    #[must_use]
    pub const fn max(&self) -> MaxCapVal {
        self.max
    }
}

impl RangeBounds<usize> for MinMaxCapVal {
    fn start_bound(&self) -> Bound<&usize> {
        self.min.start_bound()
    }
    fn end_bound(&self) -> Bound<&usize> {
        self.max.end_bound()
    }
}

impl PartialEq<ExactCapVal> for MinMaxCapVal {
    fn eq(&self, other: &ExactCapVal) -> bool {
        self.min().0 == other.0 && self.max().0 == other.0
    }
}

impl Capacity for MinMaxCapVal {
    type Error = CapError;

    fn check_compatability<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        self.min.check_compatability(iter).map_err(CapError::Underflows)?;
        self.max.check_compatability(iter).map_err(CapError::Overflows)
    }
}

impl From<ExactCapVal> for MinMaxCapVal {
    fn from(value: ExactCapVal) -> Self {
        Self { min: MinCapVal(value.0), max: MaxCapVal(value.0) }
    }
}

/// A runtime constraint specifying an exact capacity.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, From, Into, Display)]
pub struct ExactCapVal(pub usize);

impl Capacity for ExactCapVal {
    type Error = CapError;

    fn check_compatability<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        MinMaxCapVal::from(*self).check_compatability(iter)
    }
}

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
