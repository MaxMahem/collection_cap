use core::ops::RangeBounds;

/// A private module for internal implementation details.
pub mod private {
    /// A sealed trait used to restrict trait implementations to the crate.
    pub trait Sealed {}
}

/// A type with a dynamic capacity constraint, that is mutable or determined at runtime.
///
/// # Note on Compatibility
///
/// A 'compatible' iterator is one which, when fully iterated, is capable of
/// producing a count of elements that lies within the capacity constraints,
/// according to the iterator's [`Iterator::size_hint`]. Put another way, an
/// iterator's size hint provides the range of possible element counts it could
/// have when fully consumed, and to be compatible, at least one value in that
/// range must overlap with the capacity constraints.
///
/// A compatible iterator is not guaranteed to actually fit within the capacity
/// constraints when fully consumed, unless it implements [`ExactSizeIterator`],
/// or the entire range reported by [`Iterator::size_hint`] is within the capacity
/// constraints.
///
/// However, an iterator that is not compatible is guaranteed to not fit within
/// the capacity constraints when fully consumed.
///
/// See [`Iterator::size_hint`] for more details on how these bounds are
/// calculated.
pub trait Capacity: private::Sealed {
    /// The error type returned if an iterator is not compatible with the
    /// capacity constraints.
    type Error;

    /// The error type returned if an iterator is not guaranteed to fit within
    /// the capacity constraints.
    type FitError;
    /// The type representing the minimum capacity bound.
    type Min: Capacity + RangeBounds<usize>;

    /// The type representing the maximum capacity bound.
    type Max: Capacity + RangeBounds<usize>;

    /// Returns the minimum capacity constraint.
    fn min_cap(&self) -> Self::Min;

    /// Returns the maximum capacity constraint.
    fn max_cap(&self) -> Self::Max;

    /// Checks if `iter` is compatible with this capacity constraint.
    ///
    /// # Arguments
    ///
    /// * `iter` - The [`Iterator`] to check.
    ///
    /// # Errors
    ///
    /// [`Self::Error`] if the capacity constraints are not met.
    ///
    /// # Panics
    ///
    /// May panic if the iterator's size hint is not valid.
    fn check_compatibility<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized;

    /// Checks if `iter` is guaranteed to fit within the capacity constraints.
    ///
    /// Unlike [`check_compatibility`](Self::check_compatibility), which returns
    /// `Ok` if it is *possible* that the iterator fits, this method returns `Ok`
    /// only when the iterator's size hint *guarantees* it fits.
    ///
    /// # Arguments
    ///
    /// * `iter` - The [`Iterator`] to check.
    ///
    /// # Errors
    ///
    /// [`Self::FitError`] if the iterator is not guaranteed to fit within the
    /// capacity constraints.
    ///
    /// # Panics
    ///
    /// May panic if the iterator's size hint is not valid.
    fn check_fit<I>(&self, iter: &I) -> Result<(), Self::FitError>
    where
        I: Iterator + ?Sized;
}

/// A type with an associated static capacity constraint.
///
/// # Note on Compatibility
///
/// Success on a compatibility check means that the iterator's declared bounds
/// (from [`Iterator::size_hint`]) do not contradict the capacity constraints.
/// It does not guarantee that the iterator will actually be compatible during
/// iteration, as the `size_hint` only reports the minimum and maximum number
/// of elements an iterator *might* produce.
///
/// See [`Iterator::size_hint`] for more details on how these bounds are
/// calculated.
pub trait StaticCap {
    /// The type of the capacity constraint value.
    type Cap: Capacity;

    /// The static capacity constraint.
    const CAP: Self::Cap;
}

/// A type with a runtime capacity that can validate iterator compatibility.
///
/// # Note on Compatibility
///
/// Success on a compatibility check means that the iterator's declared bounds
/// (from [`Iterator::size_hint`]) do not contradict the capacity constraints.
/// It does not guarantee that the iterator will actually be compatible during
/// iteration, as the `size_hint` only reports the minimum and maximum number
/// of elements an iterator *might* produce.
///
/// See [`Iterator::size_hint`] for more details on how these bounds are
/// calculated.
pub trait VariableCap {
    /// The type of the capacity constraint value.
    type Cap: Capacity;

    /// Returns the current capacity constraint.
    fn capacity(&self) -> Self::Cap;
}

impl<V: VariableCap + ?Sized> VariableCap for &V {
    type Cap = V::Cap;

    fn capacity(&self) -> Self::Cap {
        (**self).capacity()
    }
}
