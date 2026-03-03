use crate::cap::{MaxCapVal, MinCapVal};
use crate::{ConstMaxCap, ConstMinCap, MaxCap, MinCap, StaticCap};

/// An error representing a capacity violation with variable bounds.
pub type VarCapError = CompatError<MinCapVal, MaxCapVal>;
/// An error representing a capacity violation with static bounds.
pub type StaticCapError<C> = CompatError<C, C>;

#[cfg(doc)]
use crate::Capacity;

/// A [`CompatError`] indicating that a fully consumed [`Iterator`] produces
/// fewer elements than the minimum required by a [`Capacity`] constraint.
///
/// This occurs when the maximum possible number of elements the iterator could
/// produce is less than the minimum capacity.
///
/// # Type Parameters
///
/// - `CAP`: The type of the min capacity constraint.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("Capacity underflow: max iterator size {max_size} < min capacity {min_cap}", min_cap = min_cap.min_cap().0)]
pub struct MaxUnderflow<CAP: MinCap> {
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
    /// let err = MaxUnderflow::new(5, MinCapVal(10));
    /// assert_eq!(err.max_size(), 5);
    /// assert_eq!(err.min_cap(), MinCapVal(10));
    /// ```
    #[must_use]
    pub const fn new(max_size: usize, min_cap: MinCapVal) -> Self {
        match max_size < min_cap.0 {
            true => Self { max_size, min_cap },
            false => panic!("max_size must be < min_cap"),
        }
    }

    /// Creates a new [`MaxUnderflow`] from the violating [`MinCapVal`].
    #[must_use]
    pub(crate) const fn from_cap(max_size: usize, cap: MinCapVal) -> Self {
        Self { max_size, min_cap: cap }
    }
}

impl<CAP: MinCap> MaxUnderflow<CAP> {
    /// internal wrapper.
    #[must_use]
    pub const fn from_parts_unchecked(max_size: usize, min_cap: CAP) -> Self {
        Self { max_size, min_cap }
    }
}

impl<CAP: StaticCap<Cap = CAP> + ConstMinCap> MaxUnderflow<CAP> {
    /// Internal unchecked constructor.
    #[must_use]
    pub(crate) const fn new_unchecked(max_size: usize) -> Self {
        Self { max_size, min_cap: CAP::CAP }
    }

    /// Creates a new [`MaxUnderflow`] with the given `max_size`.
    ///
    /// # Arguments
    ///
    /// - `max_size`: The maximum number of elements produced.
    ///
    /// # Panics
    ///
    /// Panics if `max_size` >= `C::MIN_CAP`.
    #[must_use]
    pub const fn new_static(max_size: usize) -> Self {
        match max_size < CAP::MIN_CAP.0 {
            true => Self::new_unchecked(max_size),
            false => panic!("max_size must be < C::MIN_CAP"),
        }
    }
}

impl<CAP: MinCap> MaxUnderflow<CAP> {
    /// The maximum number of elements the [`Iterator`] produces.
    #[must_use]
    pub const fn max_size(&self) -> usize {
        self.max_size
    }

    /// The violated min capacity constraint.
    pub fn min_cap(&self) -> MinCapVal {
        self.min_cap.min_cap()
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
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("Capacity overflow: min iterator size {min_size} > max capacity {max_cap}", max_cap = max_cap.max_cap().0)]
pub struct MinOverflow<CAP: MaxCap> {
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
    /// let err = MinOverflow::new(10, MaxCapVal(5));
    /// assert_eq!(err.min_size(), 10);
    /// assert_eq!(err.max_cap(), MaxCapVal(5));
    /// ```
    #[must_use]
    pub const fn new(min_size: usize, max_cap: MaxCapVal) -> Self {
        match min_size > max_cap.0 {
            true => Self { min_size, max_cap },
            false => panic!("min_size must be > max_cap"),
        }
    }

    /// Internal unchecked constructor.
    #[must_use]
    pub(crate) const fn from_cap(min_size: usize, cap: MaxCapVal) -> Self {
        Self { min_size, max_cap: cap }
    }
}

impl<CAP: MaxCap> MinOverflow<CAP> {
    /// internal wrapper.
    #[must_use]
    pub const fn from_parts_unchecked(min_size: usize, max_cap: CAP) -> Self {
        Self { min_size, max_cap }
    }
}

impl<CAP: StaticCap<Cap = CAP> + ConstMaxCap> MinOverflow<CAP> {
    /// Internal unchecked constructor.
    #[must_use]
    pub(crate) const fn new_unchecked(min_size: usize) -> Self {
        Self { min_size, max_cap: CAP::CAP }
    }

    /// Creates a new [`MinOverflow`] based on `min_size`.
    ///
    /// # Arguments
    ///
    /// - `min_size`: The min number of elements the [`Iterator`] can produce.
    ///
    /// # Panics
    ///
    /// Panics if `min_size` <= `C::MAX_CAP`.
    #[must_use]
    pub const fn new_static(min_size: usize) -> Self {
        match min_size > CAP::MAX_CAP.0 {
            true => Self::new_unchecked(min_size),
            false => panic!("min_size must be > C::MAX_CAP"),
        }
    }
}

impl<CAP: MaxCap> MinOverflow<CAP> {
    /// The minimum number of elements the [`Iterator`] produces.
    #[must_use]
    pub const fn min_size(&self) -> usize {
        self.min_size
    }

    /// The violated max capacity constraint.
    pub fn max_cap(&self) -> MaxCapVal {
        self.max_cap.max_cap()
    }
}

/// A violation of a [`Capacity`] constraint.
///
/// This indicates that a fully consumed [`Iterator`] is not compatible with a
/// [`Capacity`] constraint. See [`crate::Capacity#note-on-compatibility`] for
/// details.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CompatError<MIN: MinCap, MAX: MaxCap> {
    /// The minimum number of elements the iterator will produce is greater
    /// than the maximum number of elements that the capacity allows.
    #[error(transparent)]
    Overflow(#[from] MinOverflow<MAX>),

    /// The maximum number of elements the iterator will produce is less than
    /// the minimum number of elements the capacity requires.
    #[error(transparent)]
    Underflow(#[from] MaxUnderflow<MIN>),
}
