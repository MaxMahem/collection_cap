use fluent_result::bool::Then;
use size_hinter::SizeHint;

use crate::err::{TargetCapError, TargetOverflow, TargetUnderflow};
use crate::{MaxCap, MinCap, RemainingCap};

const INVALID_SIZE_HINT_MSG: &str = "Invalid size hint";

/// Represents an error that occurs when a capacity constraint is violated.
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

impl CapError {
    fn ensure_hint_can_fit(hint: SizeHint, min_cap: usize, max_cap: usize) -> Result<(), Self> {
        CapUnderflow::ensure_hint_can_fit(hint, min_cap).map_err(CapError::Underflow)?;
        CapOverflow::ensure_hint_can_fit(hint, max_cap).map_err(CapError::Overflow)
    }

    /// Ensures that `iter` can produce enough elements to satisfy the
    /// `min_cap` but will not produce more elements than the `max_cap`.
    ///
    /// # Arguments
    ///
    /// - `iter`: The [`Iterator`] to check.
    /// - `min_cap`: The minimum capacity required.
    /// - `max_cap`: The maximum capacity allowed.
    ///
    /// # Errors
    ///
    /// - [`CapError::Underflow`] if the max number of elements `iter`
    ///   can produce is less than `min_cap`.
    /// - [`CapError::Overflow`] if the min number of elements `iter`
    ///   can produce is greater than `max_cap`.
    ///
    /// Note: Success on this method does not necessarily ensure that `iter`
    /// will actually fit. This method merely ensures that `iter` does
    /// not declare that it will always produce more elements than `max_cap`,
    /// or fewer elements than `min_cap`.
    ///
    /// # Panics
    ///
    /// Panics if `iter`'s [size hint](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::err::{CapError, CapOverflow, CapUnderflow};
    /// CapError::ensure_can_fit(&(0..10), 5, 15).expect("Should fit");
    ///
    /// let err = CapError::ensure_can_fit(&(0..20), 5, 15).expect_err("Should overflow");
    /// assert_eq!(err, CapError::Overflow(CapOverflow::new(20, 15)));
    ///
    /// let err = CapError::ensure_can_fit(&(0..3), 5, 15).expect_err("Should underflow");
    /// assert_eq!(err, CapError::Underflow(CapUnderflow::new(3, 5)));
    /// ```
    pub fn ensure_can_fit<I>(iter: &I, min_cap: usize, max_cap: usize) -> Result<(), Self>
    where
        I: Iterator + ?Sized,
    {
        let hint = SizeHint::try_from(iter.size_hint()).expect(INVALID_SIZE_HINT_MSG);
        Self::ensure_hint_can_fit(hint, min_cap, max_cap)
    }

    /// Ensures that `iter` produces exactly enough elements to satisfy the
    /// `min_cap` but will not produce more elements than the `max_cap`.
    ///
    /// # Arguments
    ///
    /// - `iter`: The [`ExactSizeIterator`] to check.
    /// - `min_cap`: The minimum capacity required.
    /// - `max_cap`: The maximum capacity allowed.
    ///
    /// # Errors
    ///
    /// - [`CapError::Underflow`] if the number of elements `iter`
    ///   can produce is less than `min_cap`.
    /// - [`CapError::Overflow`] if the number of elements `iter`
    ///   can produce is greater than `max_cap`.
    ///
    /// Note: Success on this method *does* guarantee that a properly
    /// implemented [`ExactSizeIterator`] will fit.
    ///
    /// # Panics
    ///
    /// Panics if `iter`'s [size hint](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::err::{CapError, CapOverflow, CapUnderflow};
    /// CapError::ensure_fits(&(0..10), 5, 15).expect("Should fit");
    ///
    /// let err = CapError::ensure_fits(&(0..20), 5, 15).expect_err("Should overflow");
    /// assert_eq!(err, CapError::Overflow(CapOverflow::new(20, 15)));
    ///
    /// let err = CapError::ensure_fits(&(0..3), 5, 15).expect_err("Should underflow");
    /// assert_eq!(err, CapError::Underflow(CapUnderflow::new(3, 5)));
    /// ```
    pub fn ensure_fits<I>(iter: &I, min_cap: usize, max_cap: usize) -> Result<(), Self>
    where
        I: ExactSizeIterator + ?Sized,
    {
        Self::ensure_can_fit(iter, min_cap, max_cap)
    }
}

impl<C: MaxCap + ?Sized> From<TargetOverflow<C>> for CapError {
    fn from(value: TargetOverflow<C>) -> Self {
        Self::Overflow(value.into())
    }
}

impl<C: MinCap + ?Sized> From<TargetUnderflow<C>> for CapError {
    fn from(value: TargetUnderflow<C>) -> Self {
        Self::Underflow(value.into())
    }
}

impl<C: MaxCap + MinCap + ?Sized> From<TargetCapError<C>> for CapError {
    fn from(value: TargetCapError<C>) -> Self {
        match value {
            TargetCapError::Overflow(overflow) => Self::Overflow(overflow.into()),
            TargetCapError::Underflow(underflow) => Self::Underflow(underflow.into()),
        }
    }
}

/// An error indicating that an iterator will produce more elements than a
/// collection can hold.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("Capacity overflow: min iterator size {min_size} > max capacity {max_cap}")]
pub struct CapOverflow {
    /// The minimum number of elements produced.
    min_size: usize,
    /// The maximum capacity of the collection.
    max_cap: usize,
}

impl CapOverflow {
    /// Creates a new [`CapOverflow`] with the given minimum size and maximum capacity.
    ///
    /// # Arguments
    ///
    /// - `min_size`: The minimum number of elements produced.
    /// - `max_cap`: The maximum capacity of the collection.
    ///
    /// # Panics
    ///
    /// Panics if `min_size` is greater than `max_cap`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::err::CapOverflow;
    /// let err = CapOverflow::new(10, 5);
    /// assert_eq!(err.min_size(), 10);
    /// assert_eq!(err.max_cap(), 5);
    /// ```
    #[must_use]
    pub const fn new(min_size: usize, max_cap: usize) -> Self {
        assert!(min_size > max_cap, "min_size must be greater than max_cap");
        Self { min_size, max_cap }
    }

    /// The minimum number of elements produced.
    #[must_use]
    pub const fn min_size(&self) -> usize {
        self.min_size
    }

    /// The maximum capacity of the collection.
    #[must_use]
    pub const fn max_cap(&self) -> usize {
        self.max_cap
    }

    fn ensure_hint_can_fit(hint: SizeHint, max_cap: usize) -> Result<(), Self> {
        let min_size = hint.lower();
        (min_size > max_cap).then_err(Self { min_size, max_cap })
    }

    /// Ensures that the minimum number of elements `iter` produces is less
    /// than or equal to `max_cap`.
    ///
    /// # Arguments
    ///
    /// - `iter`: The [`Iterator`] to check.
    /// - `max_cap`: The maximum capacity allowed.
    ///
    /// # Errors
    ///
    /// - [`CapOverflow`] if the minimum number of elements the iterator
    ///   can produce is greater than `max_cap`.
    ///
    /// Note: Success on this method does not guarantee that `iter` will not
    /// overflow, only that it does not always produce more elements than
    /// allowed.
    ///
    /// # Panics
    ///
    /// Panics if `iter`'s [size hint](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::err::CapOverflow;
    /// CapOverflow::ensure_can_fit(&(0..15), 20).expect("Should fit");
    ///
    /// let err = CapOverflow::ensure_can_fit(&(0..25), 20).expect_err("Should overflow");
    /// assert_eq!(err.min_size(), 25);
    /// assert_eq!(err.max_cap(), 20);
    /// ```
    pub fn ensure_can_fit<I>(iter: &I, max_cap: usize) -> Result<(), Self>
    where
        I: Iterator + ?Sized,
    {
        let hint = iter.size_hint().try_into().expect(INVALID_SIZE_HINT_MSG);
        Self::ensure_hint_can_fit(hint, max_cap)
    }

    /// Ensures that the fixed number of elements `iter` produces does not
    /// exceed `max_cap`.
    ///
    /// # Arguments
    ///
    /// - `iter`: The [`ExactSizeIterator`] to check.
    /// - `max_cap`: The maximum capacity allowed.
    ///
    /// # Errors
    ///
    /// - [`CapOverflow`] if the number of elements the iterator will
    ///   produce is greater than `max_cap`.
    ///
    /// Note: Success on this method *does* guarantee that a properly
    /// implemented [`ExactSizeIterator`] will fit.
    ///
    /// # Panics
    ///
    /// Panics if `iter`'s [size hint](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::err::CapOverflow;
    /// CapOverflow::ensure_fits(&(0..15), 20).expect("Should fit");
    ///
    /// let err = CapOverflow::ensure_fits(&(0..25), 20).expect_err("Should overflow");
    /// assert_eq!(err.min_size(), 25);
    /// assert_eq!(err.max_cap(), 20);
    /// ```
    pub fn ensure_fits<I>(iter: &I, max_cap: usize) -> Result<(), Self>
    where
        I: ExactSizeIterator + ?Sized,
    {
        Self::ensure_can_fit(iter, max_cap)
    }

    /// Ensures that the minimum number of elements `iter` produces does not
    /// exceed the [remaining capacity](MaxCap::remaining_cap) of `collection`.
    ///
    /// # Arguments
    ///
    /// - `iter`: The iterator to check.
    /// - `collection`: The collection to check.
    ///
    /// # Type Parameters
    ///
    /// - `I`: The type of the iterator.
    /// - `C`: The type of the collection.
    ///
    /// # Errors
    ///
    /// - [`CapOverflow`] if the minimum number of elements the iterator
    ///   can produce is greater than the remaining capacity of the collection.
    ///
    /// Note: Success on this function does not guarantee that `iter` will not
    /// overflow `collection`'s capacity, only that it does not always produce
    /// more elements than `collection` can contain.
    ///
    /// # Panics
    ///
    /// - If `iter`'s [size hint](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::err::CapOverflow;
    /// # use arrayvec::ArrayVec;
    /// let mut vec: ArrayVec<i32, 10> = ArrayVec::new();
    /// CapOverflow::ensure_can_fit_in(&(0..5), &vec).expect("Should fit");
    ///
    /// let err = CapOverflow::ensure_can_fit_in(&(0..15), &vec)
    ///     .expect_err("Should overflow");
    /// assert_eq!(err.min_size(), 15);
    /// assert_eq!(err.max_cap(), 10);
    /// ```
    pub fn ensure_can_fit_in<I, C>(iter: &I, collection: &C) -> Result<(), Self>
    where
        I: Iterator + ?Sized,
        C: RemainingCap + ?Sized,
    {
        Self::ensure_can_fit(iter, collection.remaining_cap())
    }

    /// Ensures that the fixed number of elements `iter` produces does not
    /// exceed the [remaining capacity](MaxCap::remaining_cap) of `collection`.
    ///
    /// # Arguments
    ///
    /// - `iter`: The iterator to check.
    /// - `collection`: The collection to check.
    ///
    /// # Errors
    ///
    /// - [`CapOverflow`] if the number of elements the iterator will
    ///   produce is greater than the remaining capacity of the collection.
    ///
    /// Note: Success on this method *does* guarantee that a properly
    /// implemented [`ExactSizeIterator`] will fit.
    ///
    /// # Panics
    ///
    /// - If `iter`'s [size hint](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::err::CapOverflow;
    /// # use arrayvec::ArrayVec;
    /// let mut vec: ArrayVec<i32, 10> = ArrayVec::new();
    /// CapOverflow::ensure_fits_in(&(0..5), &vec).expect("Should fit");
    ///
    /// let err = CapOverflow::ensure_fits_in(&(0..15), &vec)
    ///     .expect_err("Should overflow");
    /// assert_eq!(err.min_size(), 15);
    /// assert_eq!(err.max_cap(), 10);
    /// ```
    pub fn ensure_fits_in<I, C>(iter: &I, collection: &C) -> Result<(), Self>
    where
        I: ExactSizeIterator + ?Sized,
        C: RemainingCap + ?Sized,
    {
        Self::ensure_fits(iter, collection.remaining_cap())
    }
}

impl<C: MaxCap + ?Sized> From<TargetOverflow<C>> for CapOverflow {
    fn from(err: TargetOverflow<C>) -> Self {
        Self { min_size: err.min_size(), max_cap: C::MAX_CAP }
    }
}

/// An error indicating that an iterator will produce fewer elements than a
/// collection requires.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("Capacity underflow: max iterator size {max_size} < min capacity {min_cap}")]
pub struct CapUnderflow {
    /// The maximum number of elements produced.
    max_size: usize,
    /// The minimum capacity of the collection.
    min_cap: usize,
}

impl CapUnderflow {
    /// Creates a new [`CapUnderflow`] with the given maximum size
    /// and minimum capacity.
    ///
    /// # Arguments
    ///
    /// - `max_size`: The maximum number of elements produced.
    /// - `min_cap`: The minimum capacity required.
    ///
    /// # Panics
    ///
    /// Panics if `max_size` is greater than `min_cap`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::err::CapUnderflow;
    /// let err = CapUnderflow::new(5, 10);
    /// assert_eq!(err.max_size(), 5);
    /// assert_eq!(err.min_cap(), 10);
    /// ```
    #[must_use]
    pub const fn new(max_size: usize, min_cap: usize) -> Self {
        assert!(max_size < min_cap, "max_size must be less than min_cap");
        Self { max_size, min_cap }
    }

    /// The maximum number of elements produced.
    #[must_use]
    pub const fn max_size(&self) -> usize {
        self.max_size
    }

    /// The minimum capacity of the collection.
    #[must_use]
    pub const fn min_cap(&self) -> usize {
        self.min_cap
    }

    fn ensure_hint_can_fit(hint: SizeHint, min_cap: usize) -> Result<(), Self> {
        hint.upper()
            .filter(|&max_size| max_size < min_cap)
            .map(|max_size| Self { max_size, min_cap })
            .map_or(Ok(()), Err)
    }

    /// Ensures the maximum number of elements `iter` produces is greater than
    /// or equal to `min_cap`.
    ///
    /// # Arguments
    ///
    /// - `iter`: The [`Iterator`] to check.
    /// - `min_cap`: The minimum capacity required.
    ///
    /// # Errors
    ///
    /// - [`CapUnderflow`] if the maximum number of elements `iter` can produce
    ///   is less than `min_cap`.
    ///
    /// Note: Success on this method does not guarantee that `iter` will
    /// produce enough elements, only that it is possible.
    ///
    /// # Panics
    ///
    /// Panics if `iter`'s [size hint](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::err::CapUnderflow;
    /// CapUnderflow::ensure_can_fit(&(0..15), 10).expect("Should fit");
    ///
    /// let err = CapUnderflow::ensure_can_fit(&(0..5), 20).expect_err("Should underflow");
    /// assert_eq!(err.max_size(), 5);
    /// assert_eq!(err.min_cap(), 20);
    /// ```
    pub fn ensure_can_fit<I>(iter: &I, min_cap: usize) -> Result<(), Self>
    where
        I: Iterator + ?Sized,
    {
        let hint = iter.size_hint().try_into().expect(INVALID_SIZE_HINT_MSG);
        Self::ensure_hint_can_fit(hint, min_cap)
    }

    /// Ensures `iter` produces enough elements to satisfy `min_cap`.
    ///
    /// # Arguments
    ///
    /// - `iter`: The [`ExactSizeIterator`] to check.
    /// - `min_cap`: The minimum capacity required.
    ///
    /// # Errors
    ///
    /// - [`CapUnderflow`] if the number of elements `iter` will produce
    ///   is less than `min_cap`.
    ///
    /// Note: Success on this method *does* guarantee that a properly
    /// implemented [`ExactSizeIterator`] will fit.
    ///
    /// # Panics
    ///
    /// Panics if `iter`'s [size hint](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::err::CapUnderflow;
    /// CapUnderflow::ensure_fits(&(0..15), 10).expect("Should fit");
    ///
    /// let err = CapUnderflow::ensure_fits(&(0..5), 20).expect_err("Should underflow");
    /// assert_eq!(err.max_size(), 5);
    /// assert_eq!(err.min_cap(), 20);
    /// ```
    pub fn ensure_fits<I>(iter: &I, min_cap: usize) -> Result<(), Self>
    where
        I: ExactSizeIterator + ?Sized,
    {
        Self::ensure_can_fit(iter, min_cap)
    }
}

impl<C: MinCap + ?Sized> From<TargetUnderflow<C>> for CapUnderflow {
    fn from(err: TargetUnderflow<C>) -> Self {
        Self { max_size: err.max_size(), min_cap: C::MIN_CAP }
    }
}
