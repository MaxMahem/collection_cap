use crate::{CapConstraint, RemainingCap, err::Overflows};

/// An extension trait for `Iterator` to check if the iterator can fit within
/// target capacity constraints.
pub trait IterCapExt {
    /// Ensures that this iterator can fit a capacity constraint `C`.
    ///
    /// # Type Parameters
    ///
    /// - `C`: The capacity constraint.
    ///
    /// # Errors
    ///
    /// - [`C::Error`](CapConstraint::Error) if the iterator cannot meet the
    ///   capacity constraints.
    ///
    /// Note: Success on this method does not guarantee that the iterator will
    /// meet `C`'s capacity constraints. It only guarantees that the iterator's
    /// [size hint](Iterator::size_hint) does not declare it is impossible to
    /// meet the capacity constraint.
    ///
    /// # Panics
    ///
    /// Panics if the iterator's [size hint](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::IterCapExt;
    /// (0..10).ensure_can_fit::<[i32; 10]>().expect("Fits MIN & MAX");
    /// (0..11).ensure_can_fit::<[i32; 10]>().expect_err("Should overflow");
    /// (0..9).ensure_can_fit::<[i32; 10]>().expect_err("Should underflow");
    /// ```
    fn ensure_can_fit<C>(&self) -> Result<(), C::Error>
    where
        Self: Iterator,
        C: CapConstraint + ?Sized,
    {
        C::check_if_can_fit(self)
    }

    /// Ensures that this iterator can fit a capacity constraint `C`.
    ///
    /// # Type Parameters
    ///
    /// - `C`: The capacity constraint.
    ///
    /// # Errors
    ///
    /// - [`C::Error`](CapConstraint::Error) if `C`'s capacity constraints are
    ///   not met.
    ///
    /// Note: Success on this method *does* guarantee that a properly
    /// implemented iterator will meet `C`'s constraint when iterated.
    ///
    /// # Panics
    ///
    /// Panics if the iterator's [size hint](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::IterCapExt;
    /// (0..10).ensure_fits::<[i32; 10]>().expect("Fits MIN & MAX");
    /// (0..11).ensure_fits::<[i32; 10]>().expect_err("Should overflow");
    /// (0..9).ensure_fits::<[i32; 10]>().expect_err("Should underflow");
    /// ```
    fn ensure_fits<C>(&self) -> Result<(), C::Error>
    where
        Self: ExactSizeIterator,
        C: CapConstraint + ?Sized,
    {
        C::check_if_can_fit(self)
    }

    /// Ensures that this iterator can fit a capacity constraint `C`.
    ///
    /// # Type Parameters
    ///
    /// - `C`: The capacity constraint.
    ///
    /// # Errors
    ///
    /// - [`Overflows`] if the minimum number of elements the iterator
    ///   can produce is greater than the remaining capacity of the collection.
    ///
    /// Note: Success on this method *does* guarantee that a properly
    /// implemented iterator will meet `C`'s constraint when iterated.
    ///
    /// # Panics
    ///
    /// Panics if the iterator's [size hint](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::IterCapExt;
    /// # use arrayvec::ArrayVec;
    /// let vec: ArrayVec<i32, 10> = ArrayVec::new();
    /// (0..10).ensure_can_fit_in(&vec).expect("Fits");
    /// (0..11).ensure_can_fit_in(&vec).expect_err("Should overflow");
    /// ```
    fn ensure_can_fit_in<C>(&self, collection: &C) -> Result<(), Overflows>
    where
        Self: Iterator,
        C: RemainingCap + ?Sized,
    {
        Overflows::ensure_can_fit_in(self, collection)
    }

    /// Ensures that this iterator can fit a capacity constraint `C`.
    ///
    /// # Type Parameters
    ///
    /// - `C`: The capacity constraint.
    ///
    /// # Errors
    ///
    /// - [`Overflows`] if the number of elements the iterator will
    ///   produce is greater than the remaining capacity of the collection.
    ///
    /// Note: Success on this method *does* guarantee that a properly
    /// implemented iterator will meet `C`'s constraint when iterated.
    ///
    /// # Panics
    ///
    /// Panics if the iterator's [size hint](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::IterCapExt;
    /// # use arrayvec::ArrayVec;
    /// let vec: ArrayVec<i32, 10> = ArrayVec::new();
    /// (0..10).ensure_fits_in(&vec).expect("Fits");
    /// (0..11).ensure_fits_in(&vec).expect_err("Should overflow");
    /// ```
    fn ensure_fits_in<C>(&self, collection: &C) -> Result<(), Overflows>
    where
        Self: ExactSizeIterator,
        C: RemainingCap + ?Sized,
    {
        Overflows::ensure_fits_in(self, collection)
    }
}

impl<I: Iterator + ?Sized> IterCapExt for I {}
