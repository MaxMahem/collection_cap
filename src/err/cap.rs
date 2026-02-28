use core::marker::PhantomData;
use core::ops::{Not, RangeBounds};

use fluent_result::bool::Then;
use size_hinter::SizeHint;

use crate::INVALID_SIZE_HINT_MSG;
use crate::cap::{MaxCapVal, MinCapVal};
use crate::{MaxCap, MinCap};

macro_rules! invalid_size_hint {
    () => {{ |_| panic!("{INVALID_SIZE_HINT_MSG}") }};
}

/// Represents a violation of `CAP`'s capacity constraint.
///
/// See [`crate::StaticCap#note-on-compatibility`] for details.
///
/// # Type Parameters
///
/// - `CAP`: The type of the capacity constraint.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CapError<CAP: MinCap + MaxCap + ?Sized> {
    /// The minimum number of elements the iterator produces is greater
    /// than the maximum number of elements that `CAP` allows.
    #[error(transparent)]
    Overflow(#[from] CapOverflow<CAP>),

    /// The maximum number of elements the iterator produces is less than
    /// the minimum number of elements `CAP` requires.
    #[error(transparent)]
    Underflow(#[from] CapUnderflow<CAP>),
}

impl<CAP: MinCap + MaxCap + ?Sized> CapError<CAP> {
    fn ensure_hint_compatible(hint: SizeHint) -> Result<(), Self> {
        CapUnderflow::ensure_hint_compatible(hint).map_err(CapError::Underflow)?;
        CapOverflow::ensure_hint_compatible(hint).map_err(CapError::Overflow)
    }

    /// Ensures that `iter` is compatible with `CAP`.
    ///
    /// Note: Does not guarantee that iteration will be within `CAP`'s capacity.
    /// See [`crate::StaticCap#note-on-compatibility`] for details.
    ///
    /// # Arguments
    ///
    /// - `iter`: The [`Iterator`] to check.
    ///
    /// # Type Parameters
    ///
    /// - `I`: The type of the [`Iterator`].
    ///
    /// # Errors
    ///
    /// - [`CapError::Underflow`] if the max number of elements `iter`
    ///   can produce is less than the [min](MinCap::MIN_CAP) capacity of `CAP`.
    /// - [`CapError::Overflow`] if the min number of elements `iter`
    ///   can produce is greater than the [max](MaxCap::MAX_CAP) capacity of `CAP`.
    ///
    /// # Panics
    ///
    /// Panics if `iter`'s [size hint](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::err::{CapError, CapOverflow, CapUnderflow};
    /// CapError::<[i32; 10]>::ensure_compatible(&(0..10)).expect("Should be compatible");
    ///
    /// CapError::<[i32; 10]>::ensure_compatible(&(0..11))
    ///     .expect_err("Should overflow");
    ///
    /// CapError::<[i32; 10]>::ensure_compatible(&(0..9))
    ///     .expect_err("Should underflow");
    /// ```
    pub fn ensure_compatible<I>(iter: &I) -> Result<(), Self>
    where
        I: Iterator + ?Sized,
    {
        iter.size_hint() //
            .try_into()
            .map_or_else(invalid_size_hint!(), Self::ensure_hint_compatible)
    }
}

/// An overflow capacity violation of `CAP`'s capacity constraint,
/// indicating that an [`Iterator`] produces more elements than `CAP` allows.
///
/// # Type Parameters
///
/// - `CAP`: The type of the capacity constraint.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("Capacity overflow: min iterator size {min_size} > max capacity {}", CAP::MAX_CAP)]
pub struct CapOverflow<CAP: MaxCap + ?Sized> {
    /// The minimum number of elements produced.
    min_size: usize,
    /// Marker for the collection type.
    _marker: PhantomData<CAP>,
}

impl<CAP: MaxCap + ?Sized> CapOverflow<CAP> {
    /// The maximum capacity of the target collection.
    pub const MAX_CAP: MaxCapVal = CAP::MAX_CAP;

    #[must_use]
    const fn new_unchecked(min_size: usize) -> Self {
        Self { min_size, _marker: PhantomData }
    }

    /// Creates a new [`CapOverflow`] based on `min_size`.
    ///
    /// # Arguments
    ///
    /// - `min_size`: The minimum number of elements the iterator produces.
    ///
    /// # Panics
    ///
    /// Panics if `min_size` is <= [`MAX_CAP`](MaxCap::MAX_CAP).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::err::CapOverflow;
    /// let err = CapOverflow::<[i32; 10]>::new(25);
    /// assert_eq!(err.min_size(), 25);
    /// ```
    #[must_use]
    pub const fn new(min_size: usize) -> Self {
        match min_size > CAP::MAX_CAP.0 {
            true => Self::new_unchecked(min_size),
            false => panic!("min_size must be > MAX_CAP"),
        }
    }

    /// The minimum number of elements the iterator produces.
    #[must_use]
    pub const fn min_size(&self) -> usize {
        self.min_size
    }

    fn ensure_hint_compatible(hint: SizeHint) -> Result<(), Self> {
        let min_size = hint.lower();
        CAP::MAX_CAP.contains(&min_size).not().then_err(Self::new_unchecked(min_size))
    }

    /// Ensures that the minimum number of elements `iter` produces does not
    /// exceed [`CAP::MAX_CAP`](MaxCap::MAX_CAP).
    ///
    /// # Arguments
    ///
    /// - `iter`: The [`Iterator`] to check.
    ///
    /// # Type Parameters
    ///
    /// - `I`: The type of the iterator.
    ///
    /// # Errors
    ///
    /// [`CapOverflow`] if the minimum number of elements the iterator
    /// can produce is greater than the maximum capacity of the collection.
    ///
    /// # Panics
    ///
    /// Panics if `iter`'s [size hint](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::err::CapOverflow;
    /// CapOverflow::<[i32; 20]>::ensure_compatible(&(0..15))
    ///     .expect("Should be compatible");
    ///
    /// CapOverflow::<[i32; 20]>::ensure_compatible(&(0..25))
    ///     .expect_err("Should overflow");
    /// ```
    pub fn ensure_compatible<I>(iter: &I) -> Result<(), Self>
    where
        I: Iterator + ?Sized,
    {
        iter.size_hint() //
            .try_into()
            .map_or_else(invalid_size_hint!(), Self::ensure_hint_compatible)
    }
}

/// A underflow violation of `CAP`'s capacity constraint,
/// indicating that an [`Iterator`] produces fewer elements than `CAP` requires.
///
/// # Type Parameters
///
/// - `CAP`: The type of the capacity constraint.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("Capacity underflow: max iterator size {max_size} < min capacity {}", CAP::MIN_CAP)]
pub struct CapUnderflow<CAP: MinCap + ?Sized> {
    /// The maximum number of elements produced.
    max_size: usize,
    /// Marker for the collection type.
    _marker: PhantomData<CAP>,
}

impl<CAP: MinCap + ?Sized> CapUnderflow<CAP> {
    /// The minimum capacity of the target collection.
    pub const MIN_CAP: MinCapVal = CAP::MIN_CAP;

    #[must_use]
    const fn new_unchecked(max_size: usize) -> Self {
        Self { max_size, _marker: PhantomData }
    }

    /// Creates a new [`CapUnderflow`] based on `max_size`.
    ///
    /// # Arguments
    ///
    /// - `max_size`: The maximum number of elements the iterator produces.
    ///
    /// # Panics
    ///
    /// Panics if `max_size` is >= the [`MIN_CAP`](MinCap::MIN_CAP) of `C`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::err::CapUnderflow;
    /// let err = CapUnderflow::<[i32; 10]>::new(5);
    /// assert_eq!(err.max_size(), 5);
    /// ```
    #[must_use]
    pub const fn new(max_size: usize) -> Self {
        match max_size < CAP::MIN_CAP.0 {
            true => Self::new_unchecked(max_size),
            false => panic!("max_size must be < MIN_CAP"),
        }
    }

    /// T5he maximum number of elements the iterator produces.
    #[must_use]
    pub const fn max_size(&self) -> usize {
        self.max_size
    }

    fn ensure_hint_compatible(hint: SizeHint) -> Result<(), Self> {
        hint.upper() //
            .filter(|&max_size| !CAP::MIN_CAP.contains(&max_size))
            .map(Self::new_unchecked)
            .map_or(Ok(()), Err)
    }

    /// Ensures `iter` can produce enough elements to satisfy
    /// [`CAP::MIN_CAP`](MinCap::MIN_CAP).
    ///
    /// # Arguments
    ///
    /// - `iter`: The [`Iterator`] to check.
    ///
    /// # Errors
    ///
    /// [`CapUnderflow`] if the maximum number of elements `iter` can produce
    /// is less than the minimum capacity of `CAP`.
    ///
    /// # Panics
    ///
    /// Panics if `iter`'s [size hint](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::err::CapUnderflow;
    /// CapUnderflow::<[i32; 15]>::ensure_compatible(&(0..15))
    ///     .expect("Should be compatible");
    ///
    /// CapUnderflow::<[i32; 20]>::ensure_compatible(&(0..5))
    ///     .expect_err("Should underflow");
    /// ```
    pub fn ensure_compatible<I>(iter: &I) -> Result<(), Self>
    where
        I: Iterator + ?Sized,
    {
        iter.size_hint() //
            .try_into()
            .map_or_else(invalid_size_hint!(), Self::ensure_hint_compatible)
    }
}
