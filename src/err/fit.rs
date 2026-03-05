use crate::cap::{MaxCapVal, MinCapVal, StaticMaxCap, StaticMinCap};
use crate::err::UpperBound;

/// A [`FitError`] indicating that an [`Iterator`]'s minimum size is
/// below the minimum capacity required.
///
/// This occurs when the minimum possible number of elements the iterator will
/// produce is less than the minimum of the constraint.
///
/// See [`Capacity#note-on-fit`] for more details.
///
/// # Type Parameters
///
/// - `CAP`: The type of the minimum capacity constraint.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("Iterator can underflow capacity: min iterator size {min_size} < {min_cap:?}")]
pub struct MinUnderflow<CAP> {
    /// The minimum number of elements the iterator will produce.
    min_size: usize,
    /// The violated minimum capacity constraint.
    min_cap: CAP,
}

impl<CAP> MinUnderflow<CAP> {
    /// Private unchecked constructor.
    #[must_use]
    pub(crate) const fn from_parts(min_size: usize, min_cap: CAP) -> Self {
        Self { min_size, min_cap }
    }

    /// The minimum number of elements the iterator will produce.
    #[must_use]
    pub const fn min_size(&self) -> usize {
        self.min_size
    }

    /// The violated minimum capacity constraint.
    pub const fn min_cap(&self) -> &CAP {
        &self.min_cap
    }
}

impl MinUnderflow<MinCapVal> {
    /// Creates a new [`MinUnderflow`].
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
            true => Self::from_parts(min_size, min_cap),
            false => panic!("min_size must be < min_cap"),
        }
    }
}

impl<const MIN: usize> MinUnderflow<StaticMinCap<MIN>> {
    /// Creates a new [`MinUnderflow`] for a static minimum capacity.
    ///
    /// # Arguments
    ///
    /// - `min_size`: The iterator's minimum size.
    ///
    /// # Panics
    ///
    /// Panics if `min_size` >= `MIN`.
    #[must_use]
    pub const fn new(min_size: usize) -> Self {
        match min_size < MIN {
            true => Self::from_parts(min_size, StaticMinCap),
            false => panic!("min_size must be < MIN"),
        }
    }
}

/// A [`FitError`] indicating that an [`Iterator`]'s maximum size
/// exceeds the maximum capacity.
///
/// This occurs when the maximum possible number of elements the iterator could
/// produce is greater than the maximum capacity.
///
/// See [`Capacity#note-on-fit`] for more details.
///
/// # Type Parameters
///
/// - `CAP`: The type of the maximum capacity constraint.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("Iterator can overflow capacity: max iterator size {max_size} > {max_cap:?}")]
pub struct MaxOverflow<CAP> {
    /// The maximum number of elements the iterator could produce.
    max_size: UpperBound,
    /// The violated maximum capacity constraint.
    max_cap: CAP,
}

impl<CAP> MaxOverflow<CAP> {
    /// Internal unchecked wrapper for a fixed max size.
    #[must_use]
    pub(crate) const fn from_parts_fixed(max_size: usize, max_cap: CAP) -> Self {
        Self { max_size: UpperBound::Fixed(max_size), max_cap }
    }

    /// Creates a new [`MaxOverflow`] based on `max_cap` where the iterator is unbounded.
    ///
    /// # Arguments
    ///
    /// - `max_cap`: The maximum capacity constraint.
    #[must_use]
    pub const fn unbounded(max_cap: CAP) -> Self {
        Self { max_size: UpperBound::Unbounded, max_cap }
    }

    /// The maximum number of elements the iterator could produce.
    #[must_use]
    pub const fn max_size(&self) -> UpperBound {
        self.max_size
    }

    /// The violated maximum capacity constraint.
    pub const fn max_cap(&self) -> &CAP {
        &self.max_cap
    }
}

impl MaxOverflow<MaxCapVal> {
    /// Creates a new [`MaxOverflow`] based on `max_size` and `max_cap`.
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
            true => Self::from_parts_fixed(max_size, max_cap),
            false => panic!("max_size must be > max_cap"),
        }
    }
}

impl<const MAX: usize> MaxOverflow<StaticMaxCap<MAX>> {
    /// Creates a new [`MaxOverflow`] for a static maximum capacity where the iterator
    /// is unbounded.
    pub const UNBOUNDED: Self = Self::unbounded(StaticMaxCap);

    /// Creates a new [`MaxOverflow`] based on `max_size` for a static maximum capacity.
    ///
    /// For an unbounded iterator, use [`Self::UNBOUNDED`] instead.
    ///
    /// # Arguments
    ///
    /// - `max_size`: The iterator's maximum size.
    ///
    /// # Panics
    ///
    /// Panics if `max_size` <= `MAX`.
    #[must_use]
    pub const fn fixed(max_size: usize) -> Self {
        match max_size > MAX {
            true => Self::from_parts_fixed(max_size, StaticMaxCap),
            false => panic!("max_size must be > MAX"),
        }
    }
}

/// A [`FitError`] indicating that an [`Iterator`]'s size hint range
/// extends both below the minimum and above the maximum capacity.
///
/// See [`Capacity#note-on-fit`] for more details.
///
/// # Type Parameters
///
/// - `MIN`: The type of the minimum capacity constraint.
/// - `MAX`: The type of the maximum capacity constraint.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{underflow}; {overflow}")]
pub struct FitErrorSpan<MIN, MAX> {
    /// The underflow portion of the violation.
    underflow: MinUnderflow<MIN>,
    /// The overflow portion of the violation.
    overflow: MaxOverflow<MAX>,
}

impl<MIN, MAX> FitErrorSpan<MIN, MAX> {
    /// Internal unchecked wrapper without statically verifying traits constraints.
    #[must_use]
    pub(crate) const fn from_parts(overflow: MaxOverflow<MAX>, underflow: MinUnderflow<MIN>) -> Self {
        Self { underflow, overflow }
    }

    /// Creates a new [`FitErrorSpan`] from `overflow` and `underflow`
    ///
    /// # Panics
    ///
    /// Panics if the capacities of `underflow` and `overflow` do not intersect.
    #[must_use]
    pub const fn new(overflow: MaxOverflow<MAX>, underflow: MinUnderflow<MIN>) -> Self {
        match overflow.max_size() {
            UpperBound::Fixed(max) if underflow.min_size() > max  // fmt
                => panic!("underflow and overflow must intersect"),
            _ => Self::from_parts(overflow, underflow),
        }
    }

    /// Returns the overflow portion of the violation.
    #[must_use]
    pub const fn overflow(&self) -> &MaxOverflow<MAX> {
        &self.overflow
    }

    /// Returns the underflow portion of the violation.
    #[must_use]
    pub const fn underflow(&self) -> &MinUnderflow<MIN> {
        &self.underflow
    }
}

/// A fit violation for a capacity constraint with both a minimum and maximum,
/// indicating that the iterator's possible range is below the minimum, above
/// the maximum capacity, or both simultaneously.
///
/// See [`Capacity#note-on-fit`] for more details.
///
/// # Type Parameters
///
/// - `MIN`: The type of the minimum capacity constraint.
/// - `MAX`: The type of the maximum capacity constraint.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum FitError<MIN, MAX> {
    /// The iterator's maximum size exceeds the maximum capacity.
    #[error(transparent)]
    Overflow(#[from] MaxOverflow<MAX>),

    /// The iterator's minimum size is below the minimum capacity.
    #[error(transparent)]
    Underflow(#[from] MinUnderflow<MIN>),

    /// The iterator's possible range extends below the minimum and above the
    /// maximum capacity simultaneously.
    #[error(transparent)]
    Both(#[from] FitErrorSpan<MIN, MAX>),
}
