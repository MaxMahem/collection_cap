use crate::{Capacity, ConstCap, VariableCap};

/// An extension trait for [`Iterator`] to check if the iterator intersects with
/// [`Capacity`] constraints.
pub trait IterCapExt: Iterator {
    /// Ensures that this [`Iterator`] is capable of producing a count of
    /// elements that intersects the associated `const` [`Capacity`] of `CAP`.
    ///
    /// See [`Capacity#note-on-intersection`] for details.
    ///
    /// # Type Parameters
    ///
    /// - `CAP`: The type of the `const` [`Capacity`] constraint, or a type
    ///   with a [`ConstCap`] constraint.
    ///
    /// # Errors
    ///
    /// [`CAP::Cap::IntersectError`](Capacity::IntersectError) if the iterator is not
    /// intersecting with the capacity constraints.
    ///
    /// # Panics
    ///
    /// Panics if the iterator's [size hint](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::IterCapExt;
    /// (0..10).ensure_intersects::<[i32; 10]>().expect("Should be intersecting");
    /// (0..5).ensure_intersects::<[i32; 10]>().expect_err("Should be incompatible");
    /// (0..100).filter(|_| false).ensure_intersects::<[i32; 10]>()
    ///     .expect("Should be a false positive");
    /// ```
    fn ensure_intersects<CAP: ConstCap>(&self) -> Result<(), <CAP::Cap as Capacity>::IntersectError> {
        CAP::CAP.check_intersects(self)
    }

    /// Ensures that this [`Iterator`] is capable of producing a count of
    /// elements that intersects the current [`Capacity`] of `cap`.
    ///
    /// See [`Capacity#note-on-intersection`] for details.
    ///
    /// # Arguments
    ///
    /// - `cap`: A variable [`Capacity`] constraint, or a collection with a
    ///   [`VariableCap`] constraint.
    ///
    /// # Type Parameters
    ///
    /// - `CAP`: The type of the collection or runtime [`Capacity`] constraint.
    ///
    /// # Errors
    ///
    /// [`CAP::Cap::IntersectError`](Capacity::IntersectError) if the iterator is not
    /// intersecting with the capacity constraints.
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
    /// (0..10).ensure_intersects_with(&array_vec).expect("Should be intersecting");
    /// (0..11).ensure_intersects_with(&array_vec).expect_err("Should be incompatible");
    /// (0..100).filter(|_| false).ensure_intersects_with(&array_vec)
    ///     .expect("Should be a false positive");
    /// ```
    fn ensure_intersects_with<CAP: VariableCap>(&self, cap: CAP) -> Result<(), <CAP::Cap as Capacity>::IntersectError> {
        cap.capacity().check_intersects(self)
    }

    /// Ensures that every possible count of elements this [`Iterator`] could
    /// produce is within the `const` [`Capacity`] of `CAP`.
    ///
    /// See [`Capacity#note-on-overlap`] for details.
    ///
    /// # Type Parameters
    ///
    /// - `CAP`: The `const` [`Capacity`] constraint type, or a type with a
    ///   [`ConstCap`] constraint.
    ///
    /// # Errors
    ///
    /// [`CAP::Cap::OverlapError`](Capacity::OverlapError) if the iterator is not guaranteed to
    /// overlap within the capacity constraints.
    ///
    /// # Panics
    ///
    /// Panics if the iterator's [size hint](Iterator::size_hint) is invalid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::IterCapExt;
    /// (0..10).ensure_overlaps::<[i32; 10]>().expect("Should overlap");
    /// (0..11).ensure_overlaps::<[i32; 10]>().expect_err("Should not overlap");
    /// (0..10).filter(|_| true).ensure_overlaps::<[i32; 10]>()
    ///     .expect_err("Should be a false negative");
    /// ```
    fn ensure_overlaps<CAP: ConstCap>(&self) -> Result<(), <CAP::Cap as Capacity>::OverlapError> {
        CAP::CAP.check_overlaps(self)
    }

    /// Ensures that every possible count of elements this [`Iterator`] could
    /// produce is within the associated [`Capacity`] of `cap`.
    ///
    /// See [`Capacity#note-on-overlap`] for details.
    ///
    /// # Arguments
    ///
    /// - `cap`: A runtime [`Capacity`] constraint, or a collection with a
    ///   [`VariableCap`] constraint.
    ///
    /// # Type Parameters
    ///
    /// - `CAP`: The type of the runtime [`Capacity`] constraint, or a type with a
    ///   [`VariableCap`] constraint.
    ///
    /// # Errors
    ///
    /// [`CAP::Cap::OverlapError`](Capacity::OverlapError) if the iterator is not
    /// guaranteed to overlap within the capacity constraints.
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
    /// (0..10).ensure_overlaps_into(&array_vec).expect("Should overlap");
    /// (0..11).ensure_overlaps_into(&array_vec).expect_err("Should not overlap");
    /// (0..11).filter(|_| true).ensure_overlaps_into(&array_vec)
    ///     .expect_err("Should be a false negative");
    /// ```
    fn ensure_overlaps_into<CAP: VariableCap>(&self, cap: CAP) -> Result<(), <CAP::Cap as Capacity>::OverlapError> {
        cap.capacity().check_overlaps(self)
    }
}

impl<I: Iterator + ?Sized> IterCapExt for I {}
