use core::marker::PhantomData;

use crate::err::UpperBound;
use crate::{MaxCap, MinCap, StaticCap};

#[cfg(doc)]
use crate::Capacity;

/// A static fit overflow violation indicating that the iterator's maximum size
/// exceeds `C::MAX_CAP`.
///
/// # Type Parameters
///
/// * `C`: The static capacity type.
#[derive(thiserror::Error, PartialEq, Eq, derive_more::Debug)]
#[error(
    "Fit overflow: max iterator size is {max_size}, exceeding max capacity {max_cap}",
    max_cap = C::MAX_CAP
)]
pub struct StaticFitOverflow<C: StaticCap + MaxCap> {
    /// The maximum number of elements the iterator could produce.
    max_size: UpperBound,
    #[debug(ignore)]
    _marker: PhantomData<C>,
}

impl<C: StaticCap + MaxCap> StaticFitOverflow<C> {
    /// Private unchecked constructor.
    pub(crate) const fn fixed_unchecked(max_size: usize) -> Self {
        Self { max_size: UpperBound::Fixed(max_size), _marker: PhantomData }
    }

    /// Creates a [`StaticFitOverflow`] from a bound `max_size`.
    ///
    /// If the iterator's max size is unbounded, use [`Self::UNBOUNDED`] instead.
    ///
    /// # Arguments
    ///
    /// - `max_size`: The iterator's maximum size.
    ///
    /// # Panics
    ///
    /// Panics if `max_size` is [`UpperBound::Fixed(n)`](UpperBound::Fixed) and `n` <= `C::MAX_CAP`.
    #[must_use]
    pub const fn fixed(max_size: usize) -> Self {
        match max_size {
            n if n > C::MAX_CAP.0 => Self::fixed_unchecked(n),
            _ => panic!("max_size must be > C::MAX_CAP"),
        }
    }

    /// A [`StaticFitOverflow`] where the iterator's maximum size is unbounded.
    pub const UNBOUNDED: Self = Self { max_size: UpperBound::Unbounded, _marker: PhantomData };

    /// The maximum number of elements the iterator could produce.
    #[must_use]
    pub const fn max_size(&self) -> UpperBound {
        self.max_size
    }
}

/// A static fit underflow violation indicating that the iterator's minimum
/// size is below `C::MIN_CAP`.
///
/// Returned by [`Capacity::check_fit`] when the size hint cannot guarantee the
/// iterator will produce at least as many elements as `C::MIN_CAP` requires.
///
/// # Type Parameters
///
/// * `C`: The type that defines the static minimum capacity constraint.
#[derive(thiserror::Error, PartialEq, Eq, derive_more::Debug)]
#[error(
    "Fit underflow: min iterator size {min_size} < min capacity {min_cap}",
    min_cap = C::MIN_CAP
)]
pub struct StaticFitUnderflow<C: StaticCap + MinCap> {
    /// The minimum number of elements the iterator will produce.
    min_size: usize,
    #[debug(ignore)]
    _marker: PhantomData<C>,
}

impl<C: StaticCap + MinCap> StaticFitUnderflow<C> {
    /// Private unchecked constructor.
    pub(crate) const fn new_unchecked(min_size: usize) -> Self {
        Self { min_size, _marker: PhantomData }
    }

    /// Creates a [`StaticFitUnderflow`] from a bound `min_size`.
    ///
    /// # Arguments
    ///
    /// - `min_size`: The iterator's minimum size.
    ///
    /// # Panics
    ///
    /// Panics if `min_size` >= `C::MIN_CAP`.
    #[must_use]
    pub const fn new(min_size: usize) -> Self {
        match min_size < C::MIN_CAP.0 {
            true => Self::new_unchecked(min_size),
            false => panic!("min_size must be < C::MIN_CAP"),
        }
    }

    /// The minimum number of elements the iterator will produce.
    #[must_use]
    pub const fn min_size(&self) -> usize {
        self.min_size
    }
}

/// A static fit violation for a capacity constraint with both a minimum and
/// maximum.
///
/// Indicates that the iterator can produce a number of elements that is either
/// below the minimum, above the maximum, or both simultaneously.
///
/// # Type Parameters
///
/// * `C`: The type that defines the static capacity constraint.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum StaticFitError<C: StaticCap + MinCap + MaxCap> {
    /// The iterator's maximum size exceeds the maximum capacity.
    #[error(transparent)]
    Overflow(#[from] StaticFitOverflow<C>),

    /// The iterator's minimum size is below the minimum capacity.
    #[error(transparent)]
    Underflow(#[from] StaticFitUnderflow<C>),

    /// The iterator's possible size range extends both below the minimum and
    /// above the maximum capacity simultaneously.
    #[error("{overflow}; {underflow}")]
    Both {
        /// The overflow portion of the violation.
        overflow: StaticFitOverflow<C>,
        /// The underflow portion of the violation.
        underflow: StaticFitUnderflow<C>,
    },
}
