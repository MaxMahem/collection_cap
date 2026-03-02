use crate::cap::{MaxCapVal, MinCapVal};
use crate::err::UpperBound;

#[cfg(doc)]
use crate::Capacity;

/// A fit overflow violation indicating that the iterator's maximum size
/// exceeds the maximum capacity.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("Iterator can overflow capacity: max iterator size {max_size} > max capacity {max_cap}")]
pub struct FitOverflow {
    /// The maximum number of elements the iterator could produce.
    max_size: UpperBound,
    /// The violated maximum capacity constraint.
    max_cap: MaxCapVal,
}

impl FitOverflow {
    /// Creates a new [`FitOverflow`] based on `max_cap` where the iterator is unbounded.
    ///
    /// # Arguments
    ///
    /// - `max_cap`: The maximum capacity constraint.
    #[must_use]
    pub const fn unbounded(max_cap: MaxCapVal) -> Self {
        Self { max_size: UpperBound::Unbounded, max_cap }
    }

    /// Creates a new [`FitOverflow`] based on `max_size` and `max_cap`.
    ///
    /// For an unbounded iterator, use [`Self::unbounded`] instead.
    ///
    /// # Arguments
    ///
    /// - `max_size`: The iterator's maximum size.
    /// - `max_cap`: The maximum capacity constraint.
    ///
    /// # Panics
    ///
    /// Panics if `max_size` <= `max_cap`.
    #[must_use]
    pub const fn fixed(max_size: usize, max_cap: MaxCapVal) -> Self {
        match max_size > max_cap.0 {
            true => Self::from_cap(max_size, max_cap),
            false => panic!("max_size must be > max_cap"),
        }
    }

    /// Private unchecked constructor.
    pub(crate) const fn from_cap(max_size: usize, max_cap: MaxCapVal) -> Self {
        Self { max_size: UpperBound::Fixed(max_size), max_cap }
    }

    /// The maximum number of elements the iterator could produce.
    #[must_use]
    pub const fn max_size(&self) -> UpperBound {
        self.max_size
    }

    /// The violated maximum capacity constraint.
    #[must_use]
    pub const fn max_cap(&self) -> MaxCapVal {
        self.max_cap
    }
}

/// A fit underflow violation indicating that the iterator's minimum size is
/// below the minimum capacity required.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("Iterator can underflow capacity: min iterator size {min_size} < min capacity {min_cap}")]
pub struct FitUnderflow {
    /// The minimum number of elements the iterator will produce.
    min_size: usize,
    /// The violated minimum capacity constraint.
    min_cap: MinCapVal,
}

impl FitUnderflow {
    /// Creates a new [`FitUnderflow`].
    ///
    /// # Arguments
    ///
    /// - `min_size`: The iterator's minimum size.
    /// - `min_cap`: The minimum capacity constraint.
    ///
    /// # Panics
    ///
    /// Panics if `min_size` >= `min_cap`.
    #[must_use]
    pub const fn new(min_size: usize, min_cap: MinCapVal) -> Self {
        match min_size < min_cap.0 {
            true => Self::from_cap(min_size, min_cap),
            false => panic!("min_size must be < min_cap"),
        }
    }

    /// Private unchecked constructor.
    pub(crate) const fn from_cap(min_size: usize, min_cap: MinCapVal) -> Self {
        Self { min_size, min_cap }
    }

    /// The minimum number of elements the iterator will produce.
    #[must_use]
    pub const fn min_size(&self) -> usize {
        self.min_size
    }

    /// The violated minimum capacity constraint.
    #[must_use]
    pub const fn min_cap(&self) -> MinCapVal {
        self.min_cap
    }
}

/// A fit violation where the iterator's size hint range extends both below
/// the minimum and above the maximum capacity simultaneously.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("{overflow}; {underflow}")]
pub struct FitBoth {
    /// The overflow portion of the violation.
    overflow: FitOverflow,
    /// The underflow portion of the violation.
    underflow: FitUnderflow,
}

impl FitBoth {
    /// Creates a new [`FitBoth`] from `overflow` and `underflow`.
    ///
    /// # Panics
    ///
    /// Panics if `underflow` > `overflow`.
    #[must_use]
    pub const fn new(overflow: FitOverflow, underflow: FitUnderflow) -> Self {
        match underflow.min_cap().0 <= overflow.max_cap().0 {
            true => Self::from_parts(overflow, underflow),
            false => panic!("Invalid capacity constraint: min_cap must be <= max_cap"),
        }
    }

    /// Private unchecked constructor.
    pub(crate) const fn from_parts(overflow: FitOverflow, underflow: FitUnderflow) -> Self {
        Self { overflow, underflow }
    }

    /// Returns the overflow portion of the violation.
    #[must_use]
    pub const fn overflow(&self) -> &FitOverflow {
        &self.overflow
    }

    /// Returns the underflow portion of the violation.
    #[must_use]
    pub const fn underflow(&self) -> &FitUnderflow {
        &self.underflow
    }
}

/// A fit violation for a capacity constraint with both a minimum and maximum,
/// indicating that the iterator's possible range is below the minimum, above
/// the maximum capacity, or both simultaneously.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum FitError {
    /// The iterator's maximum size exceeds the maximum capacity.
    #[error(transparent)]
    Overflow(#[from] FitOverflow),

    /// The iterator's minimum size is below the minimum capacity.
    #[error(transparent)]
    Underflow(#[from] FitUnderflow),

    /// The iterator's possible range extends below the minimum and above the
    /// maximum capacity simultaneously.
    #[error(transparent)]
    Both(#[from] FitBoth),
}
