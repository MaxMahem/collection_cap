use crate::cap::{MaxCapVal, MinCapVal};

/// A minimum capacity constraint.
pub trait MinCap {
    /// The minimum possible size.
    const MIN_CAP: MinCapVal;
}

/// A maximum capacity constraint.
pub trait MaxCap {
    /// The maximum possible size.
    const MAX_CAP: MaxCapVal;
}

/// A type with an associated static capacity constraints.
///
/// This trait is separate from the [`MinCap`] and [`MaxCap`] traits because a
/// type may implement both, and this trait can be used to determine how it
/// errors.
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
    /// The actual target error type returned if the constraint is violated.
    type Error;

    /// Checks if the given iterator is compatible with the capacity constraint.
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
    fn check_compatability<I>(iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized;
}

/// A variable capacity constraint that is defined at runtime.
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
    /// The error type returned if the constraint is violated.
    type Error;

    /// Checks if `iter` is compatible with this capacity constraint.
    ///
    /// # Arguments
    ///
    /// * `iter` - The [`Iterator`] to check.
    ///
    /// # Type Parameters
    ///
    /// * `I` - The type of the [`Iterator`].
    ///
    /// # Errors
    ///
    /// [`Self::Error`] if the capacity constraints are not met.
    ///
    /// # Panics
    ///
    /// May panic if the iterator's size hint is not valid.
    fn check_compatability<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized;
}

impl<CAP: VariableCap + ?Sized> VariableCap for &CAP {
    type Error = CAP::Error;

    fn check_compatability<I>(&self, iter: &I) -> Result<(), Self::Error>
    where
        I: Iterator + ?Sized,
    {
        (**self).check_compatability(iter)
    }
}
