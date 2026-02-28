use crate::{Capacity, StaticCap};

/// An extension trait for `Iterator` to check if the iterator is compatible with
/// capacity constraints.
pub trait IterCapExt {
    /// Ensures that this iterator is not incompatible with the static capacity
    /// of `C`.
    ///
    /// See [`StaticCap#note-on-compatibility`] for details.
    ///
    /// # Type Parameters
    ///
    /// - `C`: The collection type or static capacity constraint.
    ///
    /// # Errors
    ///
    /// [`C::Cap::Error`](Capacity::Error) if the iterator
    /// is not compatible with the capacity constraints.
    ///
    /// # Panics
    ///
    /// Panics if the iterator's [size hint](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::IterCapExt;
    /// (0..10).ensure_compatible::<[i32; 10]>().expect("Is compatible");
    /// (0..11).ensure_compatible::<[i32; 10]>().expect_err("Should overflow");
    /// (0..9).ensure_compatible::<[i32; 10]>().expect_err("Should underflow");
    /// ```
    fn ensure_compatible<C>(&self) -> Result<(), <C::Cap as Capacity>::Error>
    where
        Self: Iterator,
        C: StaticCap + ?Sized,
    {
        C::CAP.check_compatability(self)
    }

    /// Ensures that this iterator is not incompatible with the current capacity
    /// of `cap`.
    ///
    /// See [`Capacity#note-on-compatibility`] for details.
    ///
    /// # Arguments
    ///
    /// - `cap`: A collection or runtime capacity constraint.
    ///
    /// # Type Parameters
    ///
    /// - `CAP`: The type of the collection or runtime capacity constraint.
    ///
    /// # Errors
    ///
    /// [`CAP::Error`](Capacity::Error) if the iterator is not compatible
    /// with the capacity constraints.
    ///
    /// # Panics
    ///
    /// Panics if the iterator's [size hint](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::IterCapExt;
    /// (0..10).ensure_compatible_with(..=10).expect("Should be compatible");
    /// (0..11).ensure_compatible_with(..=10).expect_err("Should overflow");
    /// ```
    ///
    /// ```rust
    /// # use collection_cap::IterCapExt;
    /// # use arrayvec::ArrayVec;
    /// let array_vec: ArrayVec<i32, 10> = ArrayVec::new();
    /// (0..10).ensure_compatible_with(&array_vec).expect("Should be compatible");
    /// (0..11).ensure_compatible_with(&array_vec).expect_err("Should overflow");
    /// ```
    fn ensure_compatible_with<CAP>(&self, cap: CAP) -> Result<(), CAP::Error>
    where
        Self: Iterator,
        CAP: Capacity,
    {
        cap.check_compatability(self)
    }
}

impl<I: Iterator + ?Sized> IterCapExt for I {}
