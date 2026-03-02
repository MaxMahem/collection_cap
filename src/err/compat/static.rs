use core::marker::PhantomData;

use tap::{Conv, Pipe};

use crate::err::{CapError, CapOverflow, CapUnderflow};
use crate::{MaxCap, MinCap, StaticCap};

/// A static capacity violation.
///
/// This indicates than a fully consumed [`Iterator`] is not compatible with the
/// static capacity constraint [`C::CAP`](StaticCap::CAP). See
/// [`crate::Capacity#note-on-compatibility`] for details.
///
/// # Type Parameters
///
/// * `C`: The type that defines the static capacity constraint.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum StaticCapError<C: StaticCap + MaxCap + MinCap> {
    /// The minimum number of elements the iterator will produce is greater
    /// than the maximum number of elements that the capacity allows.
    #[error(transparent)]
    Overflow(#[from] StaticCapOverflow<C>),

    /// The maximum number of elements the iterator will produce is less than
    /// the minimum number of elements the capacity requires.
    #[error(transparent)]
    Underflow(#[from] StaticCapUnderflow<C>),
}

impl<C: StaticCap + MaxCap + MinCap> From<StaticCapError<C>> for CapError {
    fn from(err: StaticCapError<C>) -> Self {
        match err {
            StaticCapError::Overflow(e) => e.conv::<CapOverflow>().pipe(Self::Overflow),
            StaticCapError::Underflow(e) => e.conv::<CapUnderflow>().pipe(Self::Underflow),
        }
    }
}

/// A static capacity overflow violation.
///
/// Indicates that the minimum number of elements an [`Iterator`] produces
/// is greater than the maximum capacity defined by `C`.
///
/// The violated capacity constraint is [`C::CAP`](StaticCap::CAP).
///
/// # Type Parameters
///
/// * `C`: The type that defines the static capacity constraint.
#[derive(thiserror::Error, PartialEq, Eq, derive_more::Debug)]
#[error("Capacity overflow: min iterator size {min_size} > max capacity {max_cap}", max_cap = C::MAX_CAP)]
pub struct StaticCapOverflow<C: StaticCap + MaxCap> {
    /// The minimum number of elements produced.
    min_size: usize,
    #[debug(ignore)]
    _marker: PhantomData<C>,
}

impl<C: StaticCap + MaxCap> StaticCapOverflow<C> {
    /// Internal unchecked constructor.
    #[must_use]
    pub(crate) const fn new_unchecked(min_size: usize) -> Self {
        Self { min_size, _marker: PhantomData }
    }

    /// Creates a new [`StaticCapOverflow`] based on `min_size`.
    ///
    /// # Arguments
    ///
    /// - `min_size`: The min number of elements the [`Iterator`] can produce.
    ///
    /// # Panics
    ///
    /// Panics if `min_size` <= `C::MAX_CAP`.
    #[must_use]
    pub const fn new(min_size: usize) -> Self {
        match min_size > C::MAX_CAP.0 {
            true => Self::new_unchecked(min_size),
            false => panic!("min_size must be > C::MAX_CAP"),
        }
    }

    /// The minimum number of elements the [`Iterator`] produces.
    #[must_use]
    pub const fn min_size(&self) -> usize {
        self.min_size
    }
}

impl<C: StaticCap + MaxCap> From<StaticCapOverflow<C>> for CapOverflow {
    fn from(err: StaticCapOverflow<C>) -> Self {
        Self::from_cap(err.min_size, C::MAX_CAP)
    }
}

/// A static capacity underflow violation.
///
/// Indicates that the maximum number of elements a fully consumed [`Iterator`]
/// produces is less than the minimum capacity defined by `C`.
///
/// # Type Parameters
///
/// * `C`: The type that defines the static capacity constraint.
#[derive(thiserror::Error, PartialEq, Eq, derive_more::Debug)]
#[error("Capacity underflow: max iterator size {max_size} < min capacity {min_cap}", min_cap = C::MIN_CAP)]
pub struct StaticCapUnderflow<C: StaticCap + MinCap> {
    /// The maximum number of elements produced.
    max_size: usize,
    #[debug(ignore)]
    _marker: PhantomData<C>,
}

impl<C: StaticCap + MinCap> StaticCapUnderflow<C> {
    /// Internal unchecked constructor.
    #[must_use]
    pub(crate) const fn new_unchecked(max_size: usize) -> Self {
        Self { max_size, _marker: PhantomData }
    }

    /// Creates a new [`StaticCapUnderflow`] with the given `max_size`.
    ///
    /// # Arguments
    ///
    /// - `max_size`: The maximum number of elements produced.
    ///
    /// # Panics
    ///
    /// Panics if `max_size` >= `C::MIN_CAP`.
    #[must_use]
    pub const fn new(max_size: usize) -> Self {
        match max_size < C::MIN_CAP.0 {
            true => Self::new_unchecked(max_size),
            false => panic!("max_size must be < C::MIN_CAP"),
        }
    }

    /// The maximum number of elements the [`Iterator`] produces.
    #[must_use]
    pub const fn max_size(&self) -> usize {
        self.max_size
    }
}

impl<C: StaticCap + MinCap> From<StaticCapUnderflow<C>> for CapUnderflow {
    fn from(err: StaticCapUnderflow<C>) -> Self {
        Self::from_cap(err.max_size, C::MIN_CAP)
    }
}
