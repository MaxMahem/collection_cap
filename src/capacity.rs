use core::ops::RangeBounds;

use crate::internal::Sealed;

/// A type with a capacity constraint.
///
/// A capacity constraint represents the range of valid element counts for a
/// collection. It is useful for pre-checking if an iterator is compatible
/// before collecting the iterator into a new collection, or extending an
/// existing collection with it. See [`crate::IterCapExt`] for the primary way to
/// use these constraints with iterators.
///
/// # Note on Compatibility
///
/// A 'compatible' iterator is one which, when fully iterated, is capable of
/// producing a count of elements that lies within the capacity constraints,
/// according to the iterator's [`Iterator::size_hint`]. Put another way, an
/// iterator's size hint provides the range of possible element counts it could
/// have when fully consumed, and to be compatible, at least one value in that
/// range must be within the capacity range.
///
/// A compatible iterator is not guaranteed to actually fit within the capacity
/// constraints when fully consumed, unless it implements [`ExactSizeIterator`],
/// or the entire range reported by [`Iterator::size_hint`] is within the capacity
/// constraints (see [`Capacity::check_fit`] to test for this case instead).
///
/// However, an iterator that is not compatible is guaranteed to not fit within
/// the capacity constraints when fully consumed. That is, there can be false
/// positives, but no false negatives.
///
/// ## Examples
///
/// ```rust
/// # use collection_cap::Capacity;
/// # use collection_cap::cap::StaticMinCap;
/// let produce_10 = (0..10).filter(|_| true);
/// assert!(produce_10.size_hint() == (0, Some(10)));
/// StaticMinCap::<10>.check_compatibility(&produce_10)
///     .expect("Should be compatible");
///
/// let produce_9 = (0..9).filter(|_| true);
/// assert!(produce_9.size_hint() == (0, Some(9)));
/// StaticMinCap::<10>.check_compatibility(&produce_9)
///     .expect_err("Should not be compatible");
///
/// let produce_0 = (0..100).filter(|_| false);
/// assert!(produce_0.size_hint() == (0, Some(100)));
/// StaticMinCap::<10>.check_compatibility(&produce_0)
///     .expect("Should be a false positive");
/// ```
///
/// # Note on Fit
///
/// A iterator that 'fits' within the capacity constraints is one which, when
/// fully iterated, all possible counts of elements it could produce lies
/// within the capacity constraints, according to the iterator's [`Iterator::size_hint`].
/// Put another way, an iterator's size hint provides the range of possible element
/// counts it could have when fully consumed, and to fit, the entire range must lie
/// within the capacity constraints.
///
/// It is possible for an iterator to not 'fit' within the capacity constraints,
/// under this definition, but still generate a count of elements that lies within
/// the capacity constraints when fully consumed. But one that fits is guaranteed
/// to do so. That is, there can be false negatives, but no false positives.
///
/// ## Examples
///
/// ```rust
/// # use collection_cap::Capacity;
/// # use collection_cap::cap::StaticMaxCap;
/// let produce_0 = (0..10).filter(|_| false);
/// assert!(produce_0.size_hint() == (0, Some(10)));
/// StaticMaxCap::<10>.check_fit(&produce_0)
///     .expect("Should fit");
///
/// let produce_0 = (0..11).filter(|_| false);
/// assert!(produce_0.size_hint() == (0, Some(11)));
/// StaticMaxCap::<10>.check_fit(&produce_0)
///     .expect_err("Should not fit");
///
/// let produce_20 = (0..20).filter(|_| true);
/// assert!(produce_20.size_hint() == (0, Some(20)));
/// StaticMaxCap::<10>.check_fit(&produce_20)
///     .expect_err("Should be a false negative");
/// ```
///
/// # Note on `ExactSizeIterator`
///
/// An [`ExactSizeIterator`] that passes [`Capacity::check_compatibility`] is
/// guaranteed to fit within the capacity constraints. Likewise, one that fails
/// [`Capacity::check_fit`] is guaranteed to be incompatible.
pub trait Capacity: Sealed {
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
    /// See [the Note on Compatibility](Self#note-on-compatibility) for more
    /// details.
    ///
    /// # Arguments
    ///
    /// * `iter` - The [`Iterator`] to check.
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] if the iterator's size hint is incompatible
    /// with the capacity constraints.
    ///
    /// # Panics
    ///
    /// May panic if the iterator's size hint is not valid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::{Capacity, cap::StaticMinCap};
    /// StaticMinCap::<10>.check_compatibility(&(0..10))
    ///     .expect("Should be compatible");
    /// StaticMinCap::<10>.check_compatibility(&(0..5))
    ///     .expect_err("Should not be compatible");
    ///
    /// let produce_0 = (0..100).filter(|_| false);
    /// StaticMinCap::<10>.check_compatibility(&produce_0)
    ///     .expect("Should be a false positive");
    /// ```
    fn check_compatibility<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized;

    /// Checks if `iter` is guaranteed to fit within the capacity constraints.
    ///
    /// See [the Note on Fit](Self#note-on-fit) for more details.
    ///
    /// # Arguments
    ///
    /// * `iter` - The [`Iterator`] to check.
    ///
    /// # Errors
    ///
    /// Returns [`Self::FitError`] if the iterator is not guaranteed to fit
    /// within the capacity constraints.
    ///
    /// # Panics
    ///
    /// May panic if the iterator's size hint is not valid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::{Capacity, cap::StaticMaxCap, cap::MinCapVal};
    /// StaticMaxCap::<10>.check_fit(&(0..10)).expect("Should fit");
    /// StaticMaxCap::<10>.check_fit(&(0..11)).expect_err("Should not fit");
    ///
    /// let require_10 = MinCapVal(10);
    /// let produce_10 = (0..10).filter(|_| true);
    /// require_10.check_fit(&produce_10).expect_err("Should be a false negative");
    /// ```
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
/// See [`Capacity#note-on-compatibility`] and [`Capacity#note-on-fit`] for details.
pub trait StaticCap {
    /// The type of the capacity constraint value.
    type Cap: Capacity;

    /// The static capacity constraint.
    const CAP: Self::Cap;
}

/// A type with a mutable [`Capacity`] or one that is determined at runtime.
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

impl<V: VariableCap + ?Sized> VariableCap for &mut V {
    type Cap = V::Cap;

    fn capacity(&self) -> Self::Cap {
        (**self).capacity()
    }
}
