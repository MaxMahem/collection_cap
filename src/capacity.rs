use core::error::Error;
use core::fmt::Debug;
use core::ops::RangeBounds;

use crate::internal::Sealed;

/// A type with a [`Capacity`] constraint.
///
/// A capacity constraint represents the range of valid element counts for a
/// collection. It may be bounded or unbounded on either end, but is never
/// empty.
///
/// The primary use of capacity constraints is to pre-check if an [`Iterator`]
/// intersects before collecting the iterator into a new collection, or
/// extending an existing collection with it. See [`IterCapExt`](crate::IterCapExt)
/// for the primary way to use these constraints with iterators.
///
/// # Note on Intersection
///
/// An iterator that 'intersects' a capacity constraint is one which,
/// according to its [`Iterator::size_hint`], is capable of producing a count
/// of elements that lies within the capacity constraints when fully iterated.
///
/// That is, the range of possible element counts the size hint reports must
/// intersect with the capacity constraints at least at one point.
///
/// An intersecting iterator is not guaranteed to actually fall within the
/// capacity constraints when fully consumed (see [`Capacity::check_overlaps`] to
/// test for this case instead). However, an iterator that does *not* intersect
/// is guaranteed to *not* fall within the capacity constraints when fully
/// consumed.
///
/// ## Examples
///
/// ```rust
/// # use collection_cap::Capacity;
/// # use collection_cap::cap::ConstMinCap;
/// let produce_10 = (0..10).filter(|_| true);
/// assert!(produce_10.size_hint() == (0, Some(10)));
/// ConstMinCap::<10>.check_intersects(&produce_10)
///     .expect("Should be intersecting");
///
/// let produce_9 = (0..9).filter(|_| true);
/// assert!(produce_9.size_hint() == (0, Some(9)));
/// ConstMinCap::<10>.check_intersects(&produce_9)
///     .expect_err("Should not be intersecting");
///
/// let produce_0 = (0..100).filter(|_| false);
/// assert!(produce_0.size_hint() == (0, Some(100)));
/// ConstMinCap::<10>.check_intersects(&produce_0)
///     .expect("Should be a false positive");
/// ```
///
/// # Note on Overlap
///
/// A [`Iterator`] that 'overlaps' a capacity constraint is one which, according
/// to its [`Iterator::size_hint`], all possible counts of elements it could produce
/// lies within the range of the capacity constraint.
///
/// That is, the range of possible element counts the size hint reports must
/// be overlaped by the capacity constraint.
///
/// It is possible for an iterator to not 'overlap' the capacity constraint,
/// but still generate a count of elements that lies within the capacity constraint
/// when fully consumed. But one that overlaps is guaranteed to do so.
///
/// ## Examples
///
/// ```rust
/// # use collection_cap::Capacity;
/// # use collection_cap::cap::ConstMaxCap;
/// let produce_0 = (0..10).filter(|_| false);
/// assert!(produce_0.size_hint() == (0, Some(10)));
/// ConstMaxCap::<10>.check_overlaps(&produce_0)
///     .expect("Should overlap");
///
/// let produce_0 = (0..11).filter(|_| false);
/// assert!(produce_0.size_hint() == (0, Some(11)));
/// ConstMaxCap::<10>.check_overlaps(&produce_0)
///     .expect_err("Should not overlap");
///
/// let produce_20 = (0..20).filter(|_| true);
/// assert!(produce_20.size_hint() == (0, Some(20)));
/// ConstMaxCap::<10>.check_overlaps(&produce_20)
///     .expect_err("Should be a false negative");
/// ```
///
/// # Note on `ExactSizeIterator`
///
/// An [`ExactSizeIterator`] that passes [`Capacity::check_intersects`] is
/// guaranteed to overlap within the capacity constraints. Likewise, one that fails
/// [`Capacity::check_overlaps`] is guaranteed to be incompatible.
pub trait Capacity: Sealed + RangeBounds<usize> + Copy + Clone + Debug {
    /// The error type returned if an [`Iterator`] is does not intersect with the
    /// capacity constraints.
    type IntersectError: Error;

    /// The error type returned if an [`Iterator`] is not guaranteed to overlap within
    /// the capacity constraints.
    type OverlapError: Error;

    /// The type representing the minimum [`Capacity`] bound.
    type Min: Capacity;

    /// The type representing the maximum [`Capacity`] bound.
    type Max: Capacity;

    /// Returns the minimum [`Capacity`] constraint.
    fn min_cap(&self) -> Self::Min;

    /// Returns the maximum [`Capacity`] constraint.
    fn max_cap(&self) -> Self::Max;

    /// Returns true if `size` is within the [`Capacity`] constraints.
    fn contains_size(&self, size: usize) -> bool;

    /// Checks if `iter`'s [`Iterator::size_hint`] intersects this [`Capacity`].
    ///
    /// See [the Note on Intersection](Self#note-on-intersection) for more
    /// details.
    ///
    /// # Arguments
    ///
    /// * `iter` - The [`Iterator`] to check.
    ///
    /// # Errors
    ///
    /// [`Self::IntersectError`] if the iterator's size hint does not
    /// intersect the capacity constraints.
    ///
    /// # Panics
    ///
    /// May panic if the iterator's size hint is not valid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::{Capacity, cap::ConstMinCap};
    /// ConstMinCap::<10>.check_intersects(&(0..10))
    ///     .expect("Should be intersecting");
    /// ConstMinCap::<10>.check_intersects(&(0..5))
    ///     .expect_err("Should not be intersecting");
    ///
    /// let produce_0 = (0..100).filter(|_| false);
    /// ConstMinCap::<10>.check_intersects(&produce_0)
    ///     .expect("Should be a false positive");
    /// ```
    fn check_intersects<I>(&self, iter: &I) -> Result<(), Self::IntersectError>
    where
        I: Iterator + ?Sized;

    /// Checks if `iter`'s [`Iterator::size_hint`] overlaps this [`Capacity`].
    ///
    /// See [the Note on Overlap](Self#note-on-overlap) for more details.
    ///
    /// # Arguments
    ///
    /// * `iter` - The [`Iterator`] to check.
    ///
    /// # Errors
    ///
    /// [`Self::OverlapError`] if the [`Iterator`] is not guaranteed to overlap
    /// within the capacity constraints.
    ///
    /// # Panics
    ///
    /// May panic if the iterator's size hint is not valid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use collection_cap::{Capacity, cap::ConstMaxCap, cap::MinCapVal};
    /// ConstMaxCap::<10>.check_overlaps(&(0..10)).expect("Should overlap");
    /// ConstMaxCap::<10>.check_overlaps(&(0..11)).expect_err("Should not overlap");
    ///
    /// let require_10 = MinCapVal(10);
    /// let produce_10 = (0..10).filter(|_| true);
    /// require_10.check_overlaps(&produce_10).expect_err("Should be a false negative");
    /// ```
    fn check_overlaps<I>(&self, iter: &I) -> Result<(), Self::OverlapError>
    where
        I: Iterator + ?Sized;
}

/// A type with a `const` [`Capacity`] constraint.
pub trait ConstCap {
    /// The type of the capacity constraint value.
    type Cap: Capacity + ConstCap;

    /// The `const` [`Capacity`] constraint.
    const CAP: Self::Cap;
}

/// A type with a variable [`Capacity`].
pub trait VariableCap {
    /// The type of the capacity constraint value.
    type Cap: Capacity + VariableCap;

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
