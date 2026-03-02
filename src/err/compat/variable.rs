use crate::cap::{MaxCapVal, MinCapVal};

#[cfg(doc)]
use crate::Capacity;

/// A violation of a [`Capacity`] constraint.
///
/// This indicates that a fully consumed [`Iterator`] is not compatible with a
/// [`Capacity`] constraint. See [`crate::Capacity#note-on-compatibility`] for
/// details.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CapError {
    /// The minimum number of elements the iterator will produce is greater
    /// than the maximum number of elements that the capacity allows.
    #[error(transparent)]
    Overflow(#[from] CapOverflow),

    /// The maximum number of elements the iterator will produce is less than
    /// the minimum number of elements the capacity requires.
    #[error(transparent)]
    Underflow(#[from] CapUnderflow),
}

/// An overflow violation of a [`Capacity`] constraint indicating that the
/// minimum number of elements a fully consumed [`Iterator`] produces is greater
/// than the maximum allowed.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("Capacity overflow: min iterator size {min_size} > max capacity {max_cap}")]
pub struct CapOverflow {
    /// The minimum number of elements produced.
    min_size: usize,
    /// The maximum capacity of the collection.
    max_cap: MaxCapVal,
}

impl CapOverflow {
    /// Creates a new [`CapOverflow`] based on `min_size` and `max_cap`.
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
    /// # use collection_cap::err::CapOverflow;
    /// # use collection_cap::cap::MaxCapVal;
    /// let err = CapOverflow::new(10, MaxCapVal(5));
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

    /// The minimum number of elements the [`Iterator`] produces.
    #[must_use]
    pub const fn min_size(&self) -> usize {
        self.min_size
    }

    /// The violated max capacity constraint.
    #[must_use]
    pub const fn max_cap(&self) -> MaxCapVal {
        self.max_cap
    }
}

/// An underflow violation of a [`Capacity`] constraint indicating that a
/// fully consumed [`Iterator`] produces fewer elements than the minimum
/// required.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("Capacity underflow: max iterator size {max_size} < min capacity {min_cap}")]
pub struct CapUnderflow {
    /// The maximum number of elements produced.
    max_size: usize,
    /// The minimum capacity of the collection.
    min_cap: MinCapVal,
}

impl CapUnderflow {
    /// Creates a new [`CapUnderflow`] based on `max_size` and `min_cap`.
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
    /// # use collection_cap::err::CapUnderflow;
    /// # use collection_cap::cap::MinCapVal;
    /// let err = CapUnderflow::new(5, MinCapVal(10));
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

    /// Creates a new [`CapUnderflow`] from the violating [`MinCapVal`].
    #[must_use]
    pub(crate) const fn from_cap(max_size: usize, cap: MinCapVal) -> Self {
        Self { max_size, min_cap: cap }
    }

    /// The maximum number of elements the [`Iterator`] produces.
    #[must_use]
    pub const fn max_size(&self) -> usize {
        self.max_size
    }

    /// The violated min capacity constraint.
    #[must_use]
    pub const fn min_cap(&self) -> MinCapVal {
        self.min_cap
    }
}
