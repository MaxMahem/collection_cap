use core::ops::{Bound, Not, Range, RangeBounds, RangeInclusive};

use derive_more::{From, Into};
use fluent_result::into::{IntoOption, IntoResult};

use crate::cap::val::{ExactCapVal, MaxCapVal, MinCapVal};
use crate::err::{EmptyRange, FromRangeError, InvalidRange};
use crate::err::{IntersectError, MaxUnderflow, MinOverflow};
use crate::err::{MaxOverflow, MinUnderflow, OverlapError, OverlapErrorSpan};
use crate::internal::Ok;
use crate::{Capacity, IterExt};

/// A variable [`Capacity`] constraint specifying both a minimum and maximum [`Capacity`].
#[derive(Debug, PartialEq, Eq, Copy, Clone, Into, From)]
pub struct MinMaxCapVal {
    /// The minimum [`Capacity`] required.
    min: MinCapVal,
    /// The maximum [`Capacity`] allowed.
    max: MaxCapVal,
}

impl MinMaxCapVal {
    /// A [`Capacity`] constraint requiring exactly zero elements.
    pub const ZERO: Self = Self::new(0, 0);

    /// Internal unchecked constructor.
    #[must_use]
    pub(crate) const fn new_unchecked(min: usize, max: usize) -> Self {
        Self { min: MinCapVal(min), max: MaxCapVal(max) }
    }

    /// Creates a new [`MinMaxCapVal`] based on `min` and `max`.
    ///
    /// # Arguments
    ///
    /// - `min`: The inclusive minimum [`Capacity`] required.
    /// - `max`: The inclusive maximum [`Capacity`] allowed.
    ///
    /// # Panics
    ///
    /// Panics if `min > max`.
    #[must_use]
    pub const fn new(min: usize, max: usize) -> Self {
        match (min, max) {
            (min, max) if min > max => panic!("Invalid range (start > end)"),
            (min, max) => Self::new_unchecked(min, max),
        }
    }

    /// Returns the minimum [`Capacity`] required.
    #[must_use]
    pub const fn min(&self) -> MinCapVal {
        self.min
    }

    /// Returns the maximum [`Capacity`] allowed.
    #[must_use]
    pub const fn max(&self) -> MaxCapVal {
        self.max
    }
}

impl Capacity for MinMaxCapVal {
    type IntersectError = IntersectError<MinCapVal, MaxCapVal>;
    type OverlapError = OverlapError<MinCapVal, MaxCapVal>;
    type Min = MinCapVal;
    type Max = MaxCapVal;

    fn min_cap(&self) -> Self::Min {
        self.min()
    }

    fn max_cap(&self) -> Self::Max {
        self.max()
    }

    fn contains_size(&self, size: usize) -> bool {
        self.min.contains_size(size) && self.max.contains_size(size)
    }

    fn check_intersects<I>(&self, iter: &I) -> Result<(), Self::IntersectError>
    where
        I: Iterator + ?Sized,
    {
        match iter.valid_size_hint() {
            (min, _) if !self.max.contains_size(min) // fmt
                => MinOverflow::from_parts(min, self.max).into_err()?,
            (_, Some(max)) if !self.min.contains_size(max) // fmt
                => MaxUnderflow::from_parts(max, self.min).into_err()?,
            _ => Ok!(),
        }
    }

    fn check_overlaps<I>(&self, iter: &I) -> Result<(), Self::OverlapError>
    where
        I: Iterator + ?Sized,
    {
        let (min, max_opt) = iter.valid_size_hint();

        let underflow = self // fmt
            .min
            .contains_size(min)
            .not()
            .then(|| MinUnderflow::from_parts(min, self.min));

        let overflow = match max_opt {
            Some(max) if !self.max.contains_size(max) // fmt
                => MaxOverflow::from_parts_fixed(max, self.max).into_some(),
            None => MaxOverflow::unbounded(self.max).into_some(),
            _ => None,
        };

        match (underflow, overflow) {
            (Some(underflow), Some(overflow)) => OverlapErrorSpan::from_parts(overflow, underflow).into_err()?,
            (Some(underflow), None) => Err(underflow)?,
            (None, Some(overflow)) => Err(overflow)?,
            (None, None) => Ok!(),
        }
    }
}

crate::cap::val::impl_variable_cap_from_self!(MinMaxCapVal);

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

impl From<ExactCapVal> for MinMaxCapVal {
    fn from(value: ExactCapVal) -> Self {
        Self::new_unchecked(value.0, value.0)
    }
}

impl TryFrom<Range<usize>> for MinMaxCapVal {
    type Error = FromRangeError;
    fn try_from(value: Range<usize>) -> Result<Self, Self::Error> {
        match (value.start, value.end) {
            (start, end) if start == end => Err(EmptyRange)?,
            (start, end) if start > end => InvalidRange::new(start, end).into_err()?,
            (start, end) => Self::new_unchecked(start, end - 1).into_ok(),
        }
    }
}

impl TryFrom<RangeInclusive<usize>> for MinMaxCapVal {
    type Error = InvalidRange;
    fn try_from(value: RangeInclusive<usize>) -> Result<Self, Self::Error> {
        match (*value.start(), *value.end()) {
            (start, end) if start > end => InvalidRange::new(start, end).into_err()?,
            (start, end) => Self::new_unchecked(start, end).into_ok(),
        }
    }
}

crate::internal::impl_sealed!(MinMaxCapVal);
