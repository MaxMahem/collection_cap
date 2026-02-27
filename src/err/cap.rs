use core::marker::PhantomData;

use fluent_result::bool::Then;
use size_hinter::SizeHint;
use tap::{Pipe, TryConv};

use crate::{MaxCap, MinCap};

const INVALID_SIZE_HINT_MSG: &str = "Invalid size hint";

/// Represents an error that occurs when a capacity constraint is violated for a
/// specific collection type `C`.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CapError<C: MinCap + MaxCap + ?Sized> {
    /// The minimum number of elements the iterator will produce is greater
    /// than the maximum number of elements that the capacity allows.
    #[error(transparent)]
    Overflow(#[from] CapOverflow<C>),

    /// The maximum number of elements the iterator will produce is less than
    /// the minimum number of elements the capacity requires.
    #[error(transparent)]
    Underflow(#[from] CapUnderflow<C>),
}

impl<C: MinCap + MaxCap + ?Sized> CapError<C> {
    fn ensure_hint_can_fit(hint: SizeHint) -> Result<(), Self> {
        CapUnderflow::ensure_hint_can_fit(hint).map_err(CapError::Underflow)?;
        CapOverflow::ensure_hint_can_fit(hint).map_err(CapError::Overflow)
    }

    /// Ensures that `iter` can produce enough elements to satisfy the
    /// [min](MinCap::MIN_CAP) capacity of `C` but will not produce more
    /// elements than the [max](MaxCap::MAX_CAP) capacity of `C`.
    ///
    /// # Arguments
    ///
    /// - `iter`: The [`Iterator`] to check.
    ///
    /// # Errors
    ///
    /// - [`CapError::Underflow`] if the max number of elements `iter`
    ///   can produce is less than the [min](MinCap::MIN_CAP) capacity of `C`.
    /// - [`CapError::Overflow`] if the min number of elements `iter`
    ///   can produce is greater than the [max](MaxCap::MAX_CAP) capacity of `C`.
    ///
    /// Note: Success on this method does not necessarily ensure that `iter`
    /// will actually fit in `C`. This method merely ensures that `iter` does
    /// not declare that it will always produce more elements than `C` can
    /// contain, or fewer elements than `C` requires.
    ///
    /// # Panics
    ///
    /// Panics if `iter`'s [size hint](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::err::{CapError, CapOverflow, CapUnderflow};
    /// CapError::<[i32; 10]>::ensure_can_fit(&(0..10)).expect("Should fit");
    ///
    /// let err = CapError::<[i32; 10]>::ensure_can_fit(&(0..25))
    ///     .expect_err("Should overflow");
    /// assert_eq!(err, CapError::Overflow(CapOverflow::new(25)));
    ///
    /// let err = CapError::<[i32; 10]>::ensure_can_fit(&(0..0))
    ///     .expect_err("Should underflow");
    /// assert_eq!(err, CapError::Underflow(CapUnderflow::new(0)));
    /// ```
    pub fn ensure_can_fit<I: Iterator + ?Sized>(iter: &I) -> Result<(), Self> {
        iter.size_hint() //
            .try_conv::<SizeHint>()
            .expect(INVALID_SIZE_HINT_MSG)
            .pipe(Self::ensure_hint_can_fit)
    }

    /// Ensures that `iter` can produce exactly enough elements to satisfy the
    /// [min](MinCap::MIN_CAP) capacity of `C` but will not produce more
    /// elements than the [max](MaxCap::MAX_CAP) capacity of `C`.
    ///
    /// # Arguments
    ///
    /// - `iter`: The [`ExactSizeIterator`] to check.
    ///
    /// # Errors
    ///
    /// - [`CapError::Underflow`] if the max number of elements `iter`
    ///   can produce is less than the [min](MinCap::MIN_CAP) capacity of `C`.
    /// - [`CapError::Overflow`] if the min number of elements `iter`
    ///   can produce is greater than the [max](MaxCap::MAX_CAP) capacity of `C`.
    ///
    /// Note: Success on this method *does* guarantee that a properly
    /// implemented [`ExactSizeIterator`] will actually fit in `C`.
    ///
    /// # Panics
    ///
    /// Panics if `iter`'s [size hint](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::err::{CapError, CapOverflow, CapUnderflow};
    /// CapError::<[i32; 10]>::ensure_can_fit(&(0..10)).expect("Should fit");
    ///
    /// let err = CapError::<[i32; 10]>::ensure_can_fit(&(0..25))
    ///     .expect_err("Should overflow");
    /// assert_eq!(err, CapError::Overflow(CapOverflow::new(25)));
    ///
    /// let err = CapError::<[i32; 10]>::ensure_can_fit(&(0..0))
    ///     .expect_err("Should underflow");
    /// assert_eq!(err, CapError::Underflow(CapUnderflow::new(0)));
    /// ```
    pub fn ensure_fits<I>(iter: &I) -> Result<(), Self>
    where
        I: ExactSizeIterator + ?Sized,
    {
        Self::ensure_can_fit(iter)
    }
}

/// An error indicating that an iterator will produce more elements than a
/// specific collection `C` can hold.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("Capacity overflow: min iterator size {min_size} > max capacity {}", C::MAX_CAP)]
pub struct CapOverflow<C: MaxCap + ?Sized> {
    /// The minimum number of elements produced.
    min_size: usize,
    /// Marker for the collection type.
    _marker: PhantomData<C>,
}

impl<C: MaxCap + ?Sized> CapOverflow<C> {
    /// The maximum capacity of the target collection.
    pub const MAX_CAP: usize = C::MAX_CAP;

    /// Creates a new [`CapOverflow`] with the given minimum size.
    ///
    /// # Arguments
    ///
    /// - `min_size`: The minimum number of elements the iterator will produce.
    ///
    /// # Panics
    ///
    /// Panics if `min_size` is not greater than the
    /// [maximum capacity](MaxCap::MAX_CAP) of `C`.
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
        assert!(min_size > C::MAX_CAP, "min_size must be greater than max capacity");
        Self { min_size, _marker: PhantomData }
    }

    /// Returns the minimum number of elements the iterator will produce.
    #[must_use]
    pub const fn min_size(&self) -> usize {
        self.min_size
    }

    fn ensure_hint_can_fit(hint: SizeHint) -> Result<(), Self> {
        let min_size = hint.lower();
        (min_size > C::MAX_CAP).then_err(Self { min_size, _marker: PhantomData })
    }

    /// Ensures that the minimum number of elements `iter` produces does not
    /// exceed the [maximum capacity](MaxCap::MAX_CAP) of `C`.
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
    /// - [`CapOverflow`] if the minimum number of elements the iterator
    ///   can produce is greater than the maximum capacity of the collection.
    ///
    /// Note: Success on this method does not guarantee that `iter` will not
    /// overflow `C`'s capacity, only that it does not always produce more
    /// elements than `C` can contain.
    ///
    /// # Panics
    ///
    /// Panics if `iter`'s [size hint](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::err::CapOverflow;
    /// CapOverflow::<[i32; 20]>::ensure_can_fit(&(0..15)).expect("Should fit");
    ///
    /// let err = CapOverflow::<[i32; 20]>::ensure_can_fit(&(0..25))
    ///     .expect_err("Should overflow");
    /// assert_eq!(err, CapOverflow::new(25));
    /// ```
    pub fn ensure_can_fit<I: Iterator + ?Sized>(iter: &I) -> Result<(), Self> {
        iter.size_hint() //
            .try_conv::<SizeHint>()
            .expect(INVALID_SIZE_HINT_MSG)
            .pipe(Self::ensure_hint_can_fit)
    }

    /// Ensures that the fixed number of elements `iter` produces does not
    /// exceed the [maximum capacity](MaxCap::MAX_CAP) of `C`.
    ///
    /// # Arguments
    ///
    /// - `iter`: The [`ExactSizeIterator`] to check.
    ///
    /// # Errors
    ///
    /// - [`CapOverflow`] if the number of elements the iterator will
    ///   produce is greater than the maximum capacity of the collection.
    ///
    /// Note: Success on this method *does* guarantee that a properly
    /// implemented [`ExactSizeIterator`] will fit in `C`.
    ///
    /// # Panics
    ///
    /// Panics if `iter`'s [size hint](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::err::CapOverflow;
    /// CapOverflow::<[i32; 20]>::ensure_fits(&(0..15)).expect("Should fit");
    ///
    /// let err = CapOverflow::<[i32; 20]>::ensure_fits(&(0..25))
    ///     .expect_err("Should overflow");
    /// assert_eq!(err, CapOverflow::new(25));
    /// ```
    pub fn ensure_fits<I>(iter: &I) -> Result<(), Self>
    where
        I: ExactSizeIterator + ?Sized,
    {
        Self::ensure_can_fit(iter)
    }
}

/// An error indicating that an iterator will produce fewer elements than a
/// specific collection `C` requires.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("Capacity underflow: max iterator size {max_size} < min capacity {}", C::MIN_CAP)]
pub struct CapUnderflow<C: MinCap + ?Sized> {
    /// The maximum number of elements produced.
    max_size: usize,
    /// Marker for the collection type.
    _marker: PhantomData<C>,
}

impl<C: MinCap + ?Sized> CapUnderflow<C> {
    /// The minimum capacity of the target collection.
    pub const MIN_CAP: usize = C::MIN_CAP;

    /// Creates a new [`CapUnderflow`] with the given maximum size.
    ///
    /// # Arguments
    ///
    /// - `max_size`: The maximum number of elements the iterator will produce.
    ///
    /// # Panics
    ///
    /// Panics if `max_size` is not less than the
    /// [minimum capacity](MinCap::MIN_CAP) of `C`.
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
        assert!(max_size < C::MIN_CAP, "max_size must be less than min capacity");
        Self { max_size, _marker: PhantomData }
    }

    /// Returns the maximum number of elements the iterator will produce.
    #[must_use]
    pub const fn max_size(&self) -> usize {
        self.max_size
    }

    fn ensure_hint_can_fit(hint: SizeHint) -> Result<(), Self> {
        hint.upper()
            .filter(|&max_size| max_size < C::MIN_CAP)
            .map(|max_size| Self { max_size, _marker: PhantomData })
            .map_or(Ok(()), Err)
    }

    /// Ensures `iter` can produce enough elements to satisfy `C`'s
    /// [minimum capacity](MinCap::MIN_CAP).
    ///
    /// # Arguments
    ///
    /// - `iter`: The [`Iterator`] to check.
    ///
    /// # Errors
    ///
    /// - [`CapUnderflow`] if the maximum number of elements `iter` can produce
    ///   is less than the minimum capacity of `C`.
    ///
    /// Note: Success on this function does not guarantee that `iter` will
    /// produce as many elements as `C` requires, only that it is possible.
    ///
    /// # Panics
    ///
    /// Panics if `iter`'s [size hint](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::err::CapUnderflow;
    /// CapUnderflow::<[i32; 15]>::ensure_can_fit(&(0..15)).expect("Should fit");
    ///
    /// let err = CapUnderflow::<[i32; 20]>::ensure_can_fit(&(0..5))
    ///     .expect_err("Should underflow");
    /// assert_eq!(err, CapUnderflow::new(5));
    /// ```
    pub fn ensure_can_fit<I: Iterator + ?Sized>(iter: &I) -> Result<(), Self> {
        iter.size_hint() //
            .try_conv::<SizeHint>()
            .expect(INVALID_SIZE_HINT_MSG)
            .pipe(Self::ensure_hint_can_fit)
    }

    /// Ensures `iter` produces enough elements to satisfy `C`'s
    /// [minimum capacity](MinCap::MIN_CAP).
    ///
    /// # Arguments
    ///
    /// - `iter`: The [`ExactSizeIterator`] to check.
    ///
    /// # Errors
    ///
    /// - [`CapUnderflow`] if the number of elements `iter` will produce
    ///   is less than the minimum capacity of `C`.
    ///
    /// Note: Success on this method *does* guarantee that a properly
    /// implemented [`ExactSizeIterator`] will fit in `C`.
    ///
    /// # Panics
    ///
    /// Panics if `iter`'s [size hint](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::err::CapUnderflow;
    /// CapUnderflow::<[i32; 15]>::ensure_fits(&(0..15)).expect("Should fit");
    ///
    /// let err = CapUnderflow::<[i32; 20]>::ensure_fits(&(0..5))
    ///     .expect_err("Should underflow");
    /// assert_eq!(err, CapUnderflow::new(5));
    /// ```
    pub fn ensure_fits<I: ExactSizeIterator + ?Sized>(iter: &I) -> Result<(), Self> {
        Self::ensure_can_fit(iter)
    }
}
