use crate::{Capacity, StaticCap, VariableCap};

/// An extension trait for `Iterator` to check if the iterator is compatible with
/// capacity constraints.
pub trait IterCapExt {
    /// Ensures that this iterator is compatible with the static capacity of `C`.
    ///
    /// Compatible means that the iterator when fully consumed, could produce a
    /// count of elements that satisfies `C`'s static capacity. See
    /// [`Capacity#note-on-compatibility`] for details.
    ///
    /// # Type Parameters
    ///
    /// - `C`: The capacity constraint type (e.g., `MaxCap<10>`, `[i32; 10]`).
    ///
    /// # Errors
    ///
    /// [`C::Cap::Error`](Capacity::Error) if the iterator is not compatible with the
    /// capacity constraints.
    ///
    /// # Panics
    ///
    /// Panics if the iterator's [size hint](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::IterCapExt;
    /// (0..10).ensure_compatible::<[i32; 10]>().expect("Should be compatible");
    /// (0..5).ensure_compatible::<[i32; 10]>().expect_err("Should be incompatible");
    /// (0..100).filter(|_| false).ensure_compatible::<[i32; 10]>()
    ///     .expect("Should be a false positive");
    /// ```
    fn ensure_compatible<C>(&self) -> Result<(), <C::Cap as Capacity>::CompatError>
    where
        Self: Iterator,
        C: StaticCap,
    {
        C::CAP.check_compatibility(self)
    }

    /// Ensures that this iterator is compatible with the current capacity of
    /// `cap`.
    ///
    /// Compatible means that the iterator when fully consumed, could produce a
    /// count of elements that satisfies `cap`'s current capacity. See
    /// [`Capacity#note-on-compatibility`] for details.
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
    /// [`CAP::Cap::Error`](Capacity::Error) if the iterator is not compatible
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
    /// # use arrayvec::ArrayVec;
    /// let array_vec: ArrayVec<i32, 10> = ArrayVec::new();
    /// (0..10).ensure_compatible_with(&array_vec).expect("Should be compatible");
    /// (0..11).ensure_compatible_with(&array_vec).expect_err("Should be incompatible");
    /// (0..100).filter(|_| false).ensure_compatible_with(&array_vec)
    ///     .expect("Should be a false positive");
    /// ```
    fn ensure_compatible_with<CAP>(&self, cap: CAP) -> Result<(), <CAP::Cap as Capacity>::CompatError>
    where
        Self: Iterator,
        CAP: VariableCap,
    {
        cap.capacity().check_compatibility(self)
    }

    /// Ensures that this iterator is guaranteed to fit within the static capacity
    /// of `CAP`.
    ///
    /// 'Fit' means that all possible counts of elements this iterator could
    /// produce (according to its [`Iterator::size_hint`]) are within the capacity
    /// constraints. See [`Capacity#note-on-fit`] for details.
    ///
    /// # Type Parameters
    ///
    /// - `CAP`: The capacity constraint type (e.g., `MaxCap<10>`, `[i32; 10]`).
    ///
    /// # Errors
    ///
    /// [`CAP::Cap::FitError`](Capacity::FitError) if the iterator is not guaranteed to
    /// fit within the capacity constraints.
    ///
    /// # Panics
    ///
    /// Panics if the iterator's [size hint](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::IterCapExt;
    /// (0..10).ensure_fit::<[i32; 10]>().expect("Should fit");
    /// (0..11).ensure_fit::<[i32; 10]>().expect_err("Should not fit");
    /// (0..10).filter(|_| true).ensure_fit::<[i32; 10]>()
    ///     .expect_err("Should be a false negative");
    /// ```
    fn ensure_fit<CAP>(&self) -> Result<(), <CAP::Cap as Capacity>::FitError>
    where
        Self: Iterator,
        CAP: StaticCap,
    {
        CAP::CAP.check_fit(self)
    }

    /// Ensures that this iterator is guaranteed to fit within the current
    /// capacity of `cap`.
    ///
    /// 'Fit' means that all possible counts of elements this iterator could
    /// produce (according to its [`Iterator::size_hint`]) are within the capacity
    /// constraints. See [`Capacity#note-on-fit`] for details.
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
    /// [`CAP::Cap::FitError`](Capacity::FitError) if the iterator is not
    /// guaranteed to fit within the capacity constraints.
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
    /// let array_vec: ArrayVec<i32, 10> = ArrayVec::new();
    /// (0..10).ensure_fits_into(&array_vec).expect("Should fit");
    /// (0..11).ensure_fits_into(&array_vec).expect_err("Should not fit");
    /// (0..11).filter(|_| true).ensure_fits_into(&array_vec)
    ///     .expect_err("Should be a false negative");
    /// ```
    fn ensure_fits_into<CAP>(&self, cap: CAP) -> Result<(), <CAP::Cap as Capacity>::FitError>
    where
        Self: Iterator,
        CAP: VariableCap,
    {
        cap.capacity().check_fit(self)
    }
}

impl<I: Iterator + ?Sized> IterCapExt for I {}
