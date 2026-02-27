use crate::{StaticCap, VariableCap};

/// An extension trait for `Iterator` to check if the iterator is compatible with
/// capacity constraints.
pub trait IterCapExt {
    /// Ensures that this iterator is compatible with the capacity of `C`.
    ///
    /// Note: Does not guarantee that iteration will fit within `C`'s capacity.
    /// See [`StaticCap#note-on-compatibility`] for details.
    ///
    /// # Type Parameters
    ///
    /// - `C`: The collection or capacity constraint.
    ///
    /// # Errors
    ///
    /// [`C::Error`](StaticCap::Error) if the iterator is not compatible
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
    /// (0..10).ensure_compatible::<[i32; 10]>().expect("Is compatible");
    /// (0..11).ensure_compatible::<[i32; 10]>().expect_err("Should overflow");
    /// (0..9).ensure_compatible::<[i32; 10]>().expect_err("Should underflow");
    /// ```
    fn ensure_compatible<C>(&self) -> Result<(), C::Error>
    where
        Self: Iterator,
        C: StaticCap + ?Sized,
    {
        C::check_compatability(self)
    }

    /// Ensures that this iterator is compatible with the capacity of `cap`.
    ///
    /// Does not guarantee that iteration will fit within `cap`'s capacity. See
    /// [`VariableCap#note-on-compatibility`] for details.
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
    /// [`CAP::Error`](VariableCap::Error) if the iterator is not compatible
    /// with the capacity constraints.
    ///
    /// Note:
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
        CAP: VariableCap,
    {
        cap.check_compatability(self)
    }
}

impl<I: Iterator + ?Sized> IterCapExt for I {}
