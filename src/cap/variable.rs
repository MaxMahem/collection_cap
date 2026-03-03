use core::ops::{Bound, Not, RangeBounds};

use crate::capacity::private::Sealed;
use crate::err::{CompatError, FitBoth, FitError, FitOverflow, FitUnderflow, MaxUnderflow, MinOverflow};
use crate::{Capacity, IterExt, MaxCap, MinCap, VariableCap};
use derive_more::{Display, From, Into};
use fluent_result::into::{IntoOption, IntoResult};
use tap::Pipe;

macro_rules! impl_variable_cap_based_on_self {
    ($this:ty) => {
        impl VariableCap for $this {
            type Cap = Self;

            fn capacity(&self) -> Self {
                *self
            }
        }
    };
}

/// A runtime constraint specifying a minimum capacity.
#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, From, Into, Display)]
pub struct MinCapVal(pub usize);

impl Sealed for MinCapVal {}

impl Capacity for MinCapVal {
    type Error = MaxUnderflow<Self>;
    type FitError = FitUnderflow<Self>;

    fn check_compatibility<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        match iter.valid_size_hint() {
            (_, Some(max_size)) if !self.contains(&max_size) => MaxUnderflow::from_cap(max_size, *self).into_err(),
            _ => Ok(()),
        }
    }

    fn check_fit<I>(&self, iter: &I) -> Result<(), Self::FitError>
    where
        I: Iterator + ?Sized,
    {
        match iter.valid_size_hint() {
            (min, _) if !self.contains(&min) => FitUnderflow::from_cap(min, *self).into_err(),
            _ => Ok(()),
        }
    }
}

impl_variable_cap_based_on_self!(MinCapVal);

impl MinCap for MinCapVal {
    fn min_cap(&self) -> MinCapVal {
        *self
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

/// A runtime constraint specifying a maximum capacity.
#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, From, Into, Display)]
pub struct MaxCapVal(pub usize);

impl Sealed for MaxCapVal {}

impl Capacity for MaxCapVal {
    type Error = MinOverflow<Self>;
    type FitError = FitOverflow<Self>;

    fn check_compatibility<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        match iter.valid_size_hint() {
            (min_size, _) if !self.contains(&min_size) => MinOverflow::from_cap(min_size, *self).into_err(),
            _ => Ok(()),
        }
    }

    fn check_fit<I>(&self, iter: &I) -> Result<(), Self::FitError>
    where
        I: Iterator + ?Sized,
    {
        match iter.valid_size_hint() {
            (_, Some(max)) if !self.contains(&max) => FitOverflow::from_cap(max, *self).into_err(),
            (_, None) => FitOverflow::unbounded(*self).into_err(),
            _ => Ok(()),
        }
    }
}

impl_variable_cap_based_on_self!(MaxCapVal);

impl MaxCap for MaxCapVal {
    fn max_cap(&self) -> MaxCapVal {
        *self
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

/// A runtime constraint specifying both a minimum and maximum capacity.
#[derive(Debug, PartialEq, Eq, Copy, Clone, derive_more::Into)]
pub struct MinMaxCapVal {
    /// The minimum capacity required.
    min: MinCapVal,
    /// The maximum capacity allowed.
    max: MaxCapVal,
}

impl MinMaxCapVal {
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
    type Error = CompatError<MinCapVal, MaxCapVal>;
    type FitError = FitError<MinCapVal, MaxCapVal>;

    fn check_compatibility<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        match iter.valid_size_hint() {
            (min, _) if !self.max.contains(&min) => {
                CompatError::Overflow(MinOverflow::from_cap(min, self.max)).into_err()
            }
            (_, Some(max)) if !self.min.contains(&max) => {
                CompatError::Underflow(MaxUnderflow::from_cap(max, self.min)).into_err()
            }
            _ => Ok(()),
        }
    }

    fn check_fit<I>(&self, iter: &I) -> Result<(), Self::FitError>
    where
        I: Iterator + ?Sized,
    {
        let (min, max_opt) = iter.valid_size_hint();

        let underflow = self.min.contains(&min).not().then(|| FitUnderflow::from_cap(min, self.min));
        let overflow = match max_opt {
            Some(max) if !self.max.contains(&max) => FitOverflow::from_cap(max, self.max).into_some(),
            None => FitOverflow::unbounded(self.max).into_some(),
            _ => None,
        };

        match (underflow, overflow) {
            (Some(underflow), Some(overflow)) => FitBoth::from_parts(overflow, underflow) //
                .pipe(FitError::Both)
                .into_err(),
            (Some(underflow), None) => FitError::Underflow(underflow).into_err(),
            (None, Some(overflow)) => FitError::Overflow(overflow).into_err(),
            (None, None) => Ok(()),
        }
    }
}

impl_variable_cap_based_on_self!(MinMaxCapVal);

impl MinCap for MinMaxCapVal {
    fn min_cap(&self) -> MinCapVal {
        self.min
    }
}

impl MaxCap for MinMaxCapVal {
    fn max_cap(&self) -> MaxCapVal {
        self.max
    }
}

impl From<ExactCapVal> for MinMaxCapVal {
    fn from(value: ExactCapVal) -> Self {
        Self::new_unchecked(value.0, value.0)
    }
}

/// A runtime constraint specifying an exact capacity.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, From, Into, Display)]
pub struct ExactCapVal(pub usize);

impl Sealed for ExactCapVal {}

impl Capacity for ExactCapVal {
    type Error = CompatError<MinCapVal, MaxCapVal>;
    type FitError = FitError<MinCapVal, MaxCapVal>;

    fn check_compatibility<I>(&self, iter: &I) -> Result<(), Self::Error>
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

impl_variable_cap_based_on_self!(ExactCapVal);

impl MinCap for ExactCapVal {
    fn min_cap(&self) -> MinCapVal {
        MinCapVal(self.0)
    }
}

impl MaxCap for ExactCapVal {
    fn max_cap(&self) -> MaxCapVal {
        MaxCapVal(self.0)
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

/// A runtime constraint specifying an unbounded capacity.
///
/// This constraint is compatible with any iterator.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, From, Into, Display)]
pub struct UnboundedCapVal;

impl Sealed for UnboundedCapVal {}

impl Capacity for UnboundedCapVal {
    type Error = core::convert::Infallible;
    type FitError = core::convert::Infallible;

    /// Always returns `Ok(())` as an unbounded capacity constraint is
    /// compatible with any iterator.
    fn check_compatibility<I>(&self, _iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        Ok(())
    }

    /// Always returns `Ok(())` as an unbounded capacity constraint fits
    /// any iterator.
    fn check_fit<I>(&self, _iter: &I) -> Result<(), Self::FitError>
    where
        I: Iterator + ?Sized,
    {
        Ok(())
    }
}

impl_variable_cap_based_on_self!(UnboundedCapVal);

impl RangeBounds<usize> for UnboundedCapVal {
    fn start_bound(&self) -> Bound<&usize> {
        Bound::Unbounded
    }
    fn end_bound(&self) -> Bound<&usize> {
        Bound::Unbounded
    }
}
