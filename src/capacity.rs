/// A minimum capacity constraint.
pub trait MinCap {
    /// The minimum possible size.
    const MIN_CAP: usize;
}

/// A maximum capacity constraint.
pub trait MaxCap {
    /// The maximum possible size.
    const MAX_CAP: usize;
}

/// A mutable collection with a [maximum capacity](MaxCap::MAX_CAP) constraint.
pub trait RemainingCap: MaxCap {
    /// Gets the remaining capacity of this collection.
    fn remaining_cap(&self) -> usize;
}

/// A trait for types that have a capacity constraint.
///
/// Note: This trait is seperate from the other traits because a type may
/// implement multiple capacity constraints, and this trait can be used to
/// determine how it errors.
pub trait CapConstraint {
    /// The actual target error type returned if the constraint is violated.
    type Error;

    /// Checks if the given iterator can produce an element count that fits
    /// within the constraint.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator to check.
    ///
    /// # Type Parameters
    ///
    /// * `I` - The type of the iterator.
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] if the capacity constraints are not met.
    ///
    /// # Panics
    ///
    /// May panic if the iterator's size hint is not valid.
    fn check_if_can_fit<I>(iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized;
}

/// A capacity constraint that is defined at runtime.
///
/// Note: This trait is separate from `CapConstraint` because it requires
/// an instance of the constraint to check against, whereas `CapConstraint`
/// relies on type-level traits.
pub trait ValConstraint {
    /// The error type returned if the constraint is violated.
    type Error;

    /// Checks if the given iterator can produce an element count that fits
    /// within the constraint.
    ///
    /// # Arguments
    ///
    /// * `iter` - The iterator to check.
    ///
    /// # Type Parameters
    ///
    /// * `I` - The type of the iterator.
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] if the capacity constraints are not met.
    ///
    /// # Panics
    ///
    /// May panic if the iterator's size hint is not valid.
    fn check_if_can_fit<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized;
}
