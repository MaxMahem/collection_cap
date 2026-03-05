use core::ops::{Bound, Not, Range, RangeBounds, RangeInclusive};

use crate::cap::val::{ExactCapVal, MaxCapVal, MinCapVal};
use crate::err::{
    CompatError, EmptyRange, FitError, FitErrorSpan, InvalidRange, MaxOverflow, MaxUnderflow, MinOverflow,
    MinUnderflow, RangeError,
};
use crate::internal::{Ok, Sealed};
use crate::{Capacity, IterExt, VariableCap};
use fluent_result::into::{IntoOption, IntoResult};
use tap::Pipe;

/// A runtime constraint specifying both a minimum and maximum capacity.
#[derive(Debug, PartialEq, Eq, Copy, Clone, derive_more::Into)]
pub struct MinMaxCapVal {
    /// The minimum capacity required.
    min: MinCapVal,
    /// The maximum capacity allowed.
    max: MaxCapVal,
}

impl MinMaxCapVal {
    /// A capacity constraint requiring exactly zero elements.
    pub const ZERO: Self = Self::new_unchecked(0, 0);

    /// Internal unchecked constructor.
    #[must_use]
    pub(crate) const fn new_unchecked(min: usize, max: usize) -> Self {
        Self { min: MinCapVal(min), max: MaxCapVal(max) }
    }

    /// Creates a new [`MinMaxCapVal`] based on `min` and `max`.
    ///
    /// # Arguments
    ///
    /// - `min`: The inclusive minimum capacity required.
    /// - `max`: The inclusive maximum capacity allowed.
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

impl Sealed for MinMaxCapVal {}

impl Capacity for MinMaxCapVal {
    type CapError = CompatError<MinCapVal, MaxCapVal>;
    type FitError = FitError<MinCapVal, MaxCapVal>;
    type Min = MinCapVal;
    type Max = MaxCapVal;

    fn min_cap(&self) -> Self::Min {
        self.min()
    }

    fn max_cap(&self) -> Self::Max {
        self.max()
    }

    fn check_compatibility<I>(&self, iter: &I) -> Result<(), Self::CapError>
    where
        I: Iterator + ?Sized,
    {
        match iter.valid_size_hint() {
            (min, _) if !self.max.contains(&min) // fmt
                => MinOverflow::from_parts(min, self.max) //
                .pipe(CompatError::Overflow)
                .into_err(),
            (_, Some(max)) if !self.min.contains(&max) // fmt
                => MaxUnderflow::from_parts(max, self.min) //
                .pipe(CompatError::Underflow)
                .into_err(),
            _ => Ok!(),
        }
    }

    fn check_fit<I>(&self, iter: &I) -> Result<(), Self::FitError>
    where
        I: Iterator + ?Sized,
    {
        let (min, max_opt) = iter.valid_size_hint();

        let underflow = self
            .min
            .contains(&min) // fmt
            .not()
            .then(|| MinUnderflow::from_parts(min, self.min));

        let overflow = match max_opt {
            Some(max) if !self.max.contains(&max) // fmt
                => MaxOverflow::from_parts_fixed(max, self.max).into_some(),
            None => MaxOverflow::unbounded(self.max).into_some(),
            _ => None,
        };

        match (underflow, overflow) {
            (Some(underflow), Some(overflow)) => FitErrorSpan::from_parts(overflow, underflow) //
                .pipe(FitError::Both)
                .into_err(),
            (Some(underflow), None) => FitError::Underflow(underflow).into_err(),
            (None, Some(overflow)) => FitError::Overflow(overflow).into_err(),
            (None, None) => Ok!(),
        }
    }
}

impl VariableCap for MinMaxCapVal {
    type Cap = Self;

    fn capacity(&self) -> Self {
        *self
    }
}

impl TryFrom<Range<usize>> for MinMaxCapVal {
    type Error = RangeError;
    fn try_from(value: Range<usize>) -> Result<Self, Self::Error> {
        match (value.start, value.end) {
            (start, end) if start == end => Err(EmptyRange.into()),
            (start, end) if start > end => Err(InvalidRange.into()),
            (start, end) => Ok(Self::new_unchecked(start, end - 1)),
        }
    }
}

impl TryFrom<RangeInclusive<usize>> for MinMaxCapVal {
    type Error = InvalidRange;
    fn try_from(value: RangeInclusive<usize>) -> Result<Self, Self::Error> {
        let (start, end) = (*value.start(), *value.end());
        if start > end { Err(InvalidRange) } else { Ok(Self::new_unchecked(start, end)) }
    }
}

impl From<ExactCapVal> for MinMaxCapVal {
    fn from(value: ExactCapVal) -> Self {
        Self::new_unchecked(value.0, value.0)
    }
}
