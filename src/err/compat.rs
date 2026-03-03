use crate::Capacity;
use core::ops::RangeBounds;

use crate::cap::{MaxCapVal, MinCapVal};
use crate::capacity::StaticCap;

/// A [`CompatError`] indicating that a fully consumed [`Iterator`] produces
/// fewer elements than the minimum required by a [`Capacity`] constraint.
///
/// This occurs when the maximum possible number of elements the iterator could
/// produce is less than the minimum of the constraint.
///
/// # Type Parameters
///
/// - `CAP`: The type of the min capacity constraint.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("Capacity underflow: max iterator size {max_size} < {min_cap:?}")]
pub struct MaxUnderflow<CAP = MinCapVal> {
    /// The maximum number of elements produced.
    max_size: usize,
    /// The minimum capacity of the collection.
    min_cap: CAP,
}

impl MaxUnderflow<MinCapVal> {
    /// Creates a new [`MaxUnderflow`] based on `max_size` and `min_cap`.
    ///
    /// # Arguments
    ///
    /// - `max_size`: The maximum number of elements produced.
    /// - `min_cap`: The minimum capacity required.
    ///
    /// # Panics
    ///
    /// Panics if `max_size` is >= `min_cap`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::err::MaxUnderflow;
    /// # use collection_cap::cap::MinCapVal;
    /// let err = MaxUnderflow::<MinCapVal>::new(5, MinCapVal(10));
    /// assert_eq!(err.max_size(), 5);
    /// assert_eq!(*err.min_cap(), MinCapVal(10));
    /// ```
    #[must_use]
    pub const fn new(max_size: usize, min_cap: MinCapVal) -> Self {
        match max_size < min_cap.0 {
            true => Self { max_size, min_cap },
            false => panic!("max_size must be < min_cap"),
        }
    }
}

impl<CAP> MaxUnderflow<CAP> {
    /// Internal unchecked constructor.
    #[must_use]
    pub(crate) const fn from_parts(max_size: usize, min_cap: CAP) -> Self {
        Self { max_size, min_cap }
    }

    /// The maximum number of elements the [`Iterator`] produces.
    #[must_use]
    pub const fn max_size(&self) -> usize {
        self.max_size
    }

    /// The violated min capacity constraint.
    pub const fn min_cap(&self) -> &CAP {
        &self.min_cap
    }
}

impl<CAP: StaticCap<Cap = CAP> + Capacity> MaxUnderflow<CAP> {
    /// Internal unchecked constructor.
    #[must_use]
    pub(crate) const fn new_unchecked(max_size: usize) -> Self {
        Self { max_size, min_cap: CAP::CAP }
    }

    /// Creates a new [`MaxUnderflow`] for a static capacity.
    ///
    /// # Panics
    ///
    /// Panics if the capacity check is not violated.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::err::MaxUnderflow;
    /// # use collection_cap::cap::StaticMinCap;
    /// # use collection_cap::StaticCap;
    /// let err = MaxUnderflow::<StaticMinCap<10>>::new(5);
    /// assert_eq!(err.max_size(), 5);
    /// assert_eq!(*err.min_cap(), StaticMinCap::<10>::CAP);
    /// ```
    #[must_use]
    pub fn new(max_size: usize) -> Self {
        match CAP::CAP.min_cap().contains(&max_size) {
            true => panic!("max_size must be < C::MIN_CAP"),
            false => Self::new_unchecked(max_size),
        }
    }
}

/// A [`CompatError`] indicating that a fully consumed [`Iterator`] produces
/// more elements than the maximum allowed by a [`Capacity`] constraint.
///
/// This occurs when the minimum possible number of elements the iterator will
/// produce is greater than the maximum capacity.
///
/// # Type Parameters
///
/// - `CAP`: The type of the max capacity constraint.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("Capacity overflow: min iterator size {min_size} > {max_cap:?}")]
pub struct MinOverflow<CAP> {
    /// The minimum number of elements produced.
    min_size: usize,
    /// The maximum capacity of the collection.
    max_cap: CAP,
}

impl MinOverflow<MaxCapVal> {
    /// Creates a new [`MinOverflow`] based on `min_size` and `max_cap`.
    ///
    /// # Arguments
    ///
    /// - `min_size`: The minimum number of elements the [`Iterator`] will produce.
    /// - `max_cap`: The maximum capacity constraint.
    ///
    /// # Panics
    ///
    /// Panics if `min_size` <= `max_cap`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::err::MinOverflow;
    /// # use collection_cap::cap::MaxCapVal;
    /// let err = MinOverflow::<MaxCapVal>::new(10, MaxCapVal(5));
    /// assert_eq!(err.min_size(), 10);
    /// assert_eq!(*err.max_cap(), MaxCapVal(5));
    /// ```
    #[must_use]
    pub const fn new(min_size: usize, max_cap: MaxCapVal) -> Self {
        match min_size > max_cap.0 {
            true => Self { min_size, max_cap },
            false => panic!("min_size must be > max_cap"),
        }
    }
}

impl<CAP> MinOverflow<CAP> {
    /// Internal unchecked constructor.
    #[must_use]
    pub(crate) const fn from_parts(min_size: usize, max_cap: CAP) -> Self {
        Self { min_size, max_cap }
    }

    /// The minimum number of elements the [`Iterator`] produces.
    #[must_use]
    pub const fn min_size(&self) -> usize {
        self.min_size
    }

    /// The violated max capacity constraint.
    pub const fn max_cap(&self) -> &CAP {
        &self.max_cap
    }
}

impl<CAP: StaticCap<Cap = CAP> + Capacity> MinOverflow<CAP> {
    /// Internal unchecked constructor.
    #[must_use]
    pub(crate) const fn new_unchecked(min_size: usize) -> Self {
        Self { min_size, max_cap: CAP::CAP }
    }

    /// Creates a new [`MinOverflow`] for a static capacity.
    ///
    /// # Panics
    ///
    /// Panics if the capacity check is not violated.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::err::MinOverflow;
    /// # use collection_cap::cap::StaticMaxCap;
    /// # use collection_cap::StaticCap;
    /// let err = MinOverflow::<StaticMaxCap<10>>::new(15);
    /// assert_eq!(err.min_size(), 15);
    /// assert_eq!(*err.max_cap(), StaticMaxCap::<10>::CAP);
    /// ```
    #[must_use]
    pub fn new(min_size: usize) -> Self {
        match CAP::CAP.max_cap().contains(&min_size) {
            true => panic!("min_size must be > C::MAX_CAP"),
            false => Self::new_unchecked(min_size),
        }
    }
}

/// A violation of a [`Capacity`] constraint.
///
/// This indicates that a fully consumed [`Iterator`] is not compatible with a
/// [`Capacity`] constraint, either because the fully consumed iterator will produce
/// too many or too few elements.
#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum CompatError<MIN, MAX> {
    /// The minimum number of elements the iterator will produce is greater
    /// than the maximum number of elements that the capacity allows.
    #[error(transparent)]
    Overflow(#[from] MinOverflow<MAX>),

    /// The maximum number of elements the iterator will produce is less than
    /// the minimum number of elements the capacity requires.
    #[error(transparent)]
    Underflow(#[from] MaxUnderflow<MIN>),
}
