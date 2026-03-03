use crate::cap::{MaxCapVal, MinCapVal};
use crate::err::UpperBound;
use crate::{ConstMaxCap, ConstMinCap, MaxCap, MinCap, StaticCap};

/// An error representing a fit violation with variable bounds.
pub type VarFitError = FitError<MinCapVal, MaxCapVal>;
/// An error representing a fit violation with static bounds.
pub type StaticFitError<C> = FitError<C, C>;

#[cfg(doc)]
use crate::Capacity;

/// A fit underflow violation indicating that the iterator's minimum size is
/// below the minimum capacity required.
///
/// # Type Parameter
///
/// - `CAP`: The type of the minimum capacity constraint.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("Iterator can underflow capacity: min iterator size {min_size} < min capacity {min_cap}", min_cap = min_cap.min_cap().0)]
pub struct FitUnderflow<CAP: MinCap> {
    /// The minimum number of elements the iterator will produce.
    min_size: usize,
    /// The violated minimum capacity constraint.
    min_cap: CAP,
}

impl FitUnderflow<MinCapVal> {
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
}

impl<CAP: MinCap> FitUnderflow<CAP> {
    /// Internal unchecked wrapper.
    #[must_use]
    pub const fn from_parts_unchecked(min_size: usize, min_cap: CAP) -> Self {
        Self { min_size, min_cap }
    }
}

impl<CAP: StaticCap<Cap = CAP> + ConstMinCap> FitUnderflow<CAP> {
    /// Private unchecked constructor.
    pub(crate) const fn new_unchecked(min_size: usize) -> Self {
        Self { min_size, min_cap: CAP::CAP }
    }

    /// Creates a [`FitUnderflow`] from a bound `min_size`.
    ///
    /// # Arguments
    ///
    /// - `min_size`: The iterator's minimum size.
    ///
    /// # Panics
    ///
    /// Panics if `min_size` >= `C::MIN_CAP`.
    #[must_use]
    pub const fn new_static(min_size: usize) -> Self {
        match min_size < CAP::MIN_CAP.0 {
            true => Self::new_unchecked(min_size),
            false => panic!("min_size must be < C::MIN_CAP"),
        }
    }
}

impl<CAP: MinCap> FitUnderflow<CAP> {
    /// The minimum number of elements the iterator will produce.
    #[must_use]
    pub const fn min_size(&self) -> usize {
        self.min_size
    }

    /// The violated minimum capacity constraint.
    pub fn min_cap(&self) -> MinCapVal {
        self.min_cap.min_cap()
    }
}

/// A fit overflow violation indicating that the iterator's maximum size
/// exceeds the maximum capacity.
///
/// # Type Parameter
///
/// - `CAP`: The type of the maximum capacity constraint.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("Iterator can overflow capacity: max iterator size {max_size} > max capacity {max_cap}", max_cap = max_cap.max_cap().0)]
pub struct FitOverflow<CAP: MaxCap> {
    /// The maximum number of elements the iterator could produce.
    max_size: UpperBound,
    /// The violated maximum capacity constraint.
    max_cap: CAP,
}

impl FitOverflow<MaxCapVal> {
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
}

impl<CAP: MaxCap> FitOverflow<CAP> {
    /// Internal unchecked wrapper.
    #[must_use]
    pub const fn from_parts_unchecked(max_size: usize, max_cap: CAP) -> Self {
        Self { max_size: UpperBound::Fixed(max_size), max_cap }
    }
}

impl<CAP: StaticCap<Cap = CAP> + ConstMaxCap> FitOverflow<CAP> {
    /// Private unchecked constructor.
    pub(crate) const fn fixed_unchecked(max_size: usize) -> Self {
        Self { max_size: UpperBound::Fixed(max_size), max_cap: CAP::CAP }
    }

    /// Creates a [`FitOverflow`] from a bound `max_size`.
    ///
    /// If the iterator's max size is unbounded, use [`Self::UNBOUNDED`] instead.
    ///
    /// # Arguments
    ///
    /// - `max_size`: The iterator's maximum size.
    ///
    /// # Panics
    ///
    /// Panics if `max_size` is [`UpperBound::Fixed(n)`](UpperBound::Fixed)
    /// and `n` <= `CAP::MAX_CAP`.
    #[must_use]
    pub const fn fixed_static(max_size: usize) -> Self {
        match max_size {
            n if n > CAP::MAX_CAP.0 => Self::fixed_unchecked(n),
            _ => panic!("max_size must be > C::MAX_CAP"),
        }
    }

    /// A [`FitOverflow`] where the iterator's maximum size is unbounded.
    pub const UNBOUNDED: Self = Self { max_size: UpperBound::Unbounded, max_cap: CAP::CAP };
}

impl<CAP: MaxCap> FitOverflow<CAP> {
    /// The maximum number of elements the iterator could produce.
    #[must_use]
    pub const fn max_size(&self) -> UpperBound {
        self.max_size
    }

    /// The violated maximum capacity constraint.
    pub fn max_cap(&self) -> MaxCapVal {
        self.max_cap.max_cap()
    }
}

/// A fit violation where the iterator's size hint range extends both below
/// the minimum and above the maximum capacity simultaneously.
///
/// # Type Parameters
///
/// - `MIN`: The type of the minimum capacity constraint.
/// - `MAX`: The type of the maximum capacity constraint.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("{overflow}; {underflow}")]
pub struct FitBoth<MIN: MinCap, MAX: MaxCap> {
    /// The overflow portion of the violation.
    overflow: FitOverflow<MAX>,
    /// The underflow portion of the violation.
    underflow: FitUnderflow<MIN>,
}

impl FitBoth<MinCapVal, MaxCapVal> {
    /// Creates a new [`FitBoth`] from `overflow` and `underflow`.
    ///
    /// # Panics
    ///
    /// Panics if `underflow` > `overflow`.
    #[must_use]
    pub fn new(overflow: FitOverflow<MaxCapVal>, underflow: FitUnderflow<MinCapVal>) -> Self {
        match underflow.min_cap().0 <= overflow.max_cap().0 {
            true => Self::from_parts(overflow, underflow),
            false => panic!("Invalid capacity constraint: min_cap must be <= max_cap"),
        }
    }

    /// Private unchecked constructor.
    pub(crate) const fn from_parts(overflow: FitOverflow<MaxCapVal>, underflow: FitUnderflow<MinCapVal>) -> Self {
        Self { overflow, underflow }
    }
}

impl<CAP: StaticCap<Cap = CAP> + ConstMinCap + ConstMaxCap> FitBoth<CAP, CAP> {
    /// Private unchecked constructor.
    pub(crate) const fn from_parts_static(overflow: FitOverflow<CAP>, underflow: FitUnderflow<CAP>) -> Self {
        Self { overflow, underflow }
    }
}

impl<MIN: MinCap, MAX: MaxCap> FitBoth<MIN, MAX> {
    /// Internal unchecked wrapper.
    #[must_use]
    pub const fn from_parts_unchecked(overflow: FitOverflow<MAX>, underflow: FitUnderflow<MIN>) -> Self {
        Self { overflow, underflow }
    }
}

impl<MIN: MinCap, MAX: MaxCap> FitBoth<MIN, MAX> {
    /// Returns the overflow portion of the violation.
    #[must_use]
    pub const fn overflow(&self) -> &FitOverflow<MAX> {
        &self.overflow
    }

    /// Returns the underflow portion of the violation.
    #[must_use]
    pub const fn underflow(&self) -> &FitUnderflow<MIN> {
        &self.underflow
    }
}

/// A fit violation for a capacity constraint with both a minimum and maximum,
/// indicating that the iterator's possible range is below the minimum, above
/// the maximum capacity, or both simultaneously.
///
/// # Type Parameters
///
/// - `MIN`: The type of the minimum capacity constraint.
/// - `MAX`: The type of the maximum capacity constraint.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum FitError<MIN: MinCap, MAX: MaxCap> {
    /// The iterator's maximum size exceeds the maximum capacity.
    #[error(transparent)]
    Overflow(#[from] FitOverflow<MAX>),

    /// The iterator's minimum size is below the minimum capacity.
    #[error(transparent)]
    Underflow(#[from] FitUnderflow<MIN>),

    /// The iterator's possible range extends below the minimum and above the
    /// maximum capacity simultaneously.
    #[error(transparent)]
    Both(#[from] FitBoth<MIN, MAX>),
}
