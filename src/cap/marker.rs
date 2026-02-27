use crate::err::{CapError, CapOverflow, CapUnderflow};
use crate::{CapConstraint, MaxCap, MinCap};

/// A marker for a minimum capacity constraint.
///
/// # Type Parameters
///
/// * `MIN`: The minimum size of the constraint.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct MinCapMarker<const MIN: usize> {}

impl<const MIN: usize> MinCap for MinCapMarker<MIN> {
    const MIN_CAP: usize = MIN;
}

impl<const MIN: usize> CapConstraint for MinCapMarker<MIN> {
    type Error = CapUnderflow<Self>;

    fn check_if_can_fit<I: Iterator + ?Sized>(iter: &I) -> Result<(), Self::Error> {
        CapUnderflow::ensure_can_fit(iter)
    }
}

/// A marker for a maximum capacity constraint.
///
/// # Type Parameters
///
/// * `MAX`: The maximum size of the constraint.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct MaxCapMarker<const MAX: usize> {}

impl<const MAX: usize> MaxCap for MaxCapMarker<MAX> {
    const MAX_CAP: usize = MAX;
}

impl<const MAX: usize> CapConstraint for MaxCapMarker<MAX> {
    type Error = CapOverflow<Self>;

    fn check_if_can_fit<I: Iterator + ?Sized>(iter: &I) -> Result<(), Self::Error> {
        CapOverflow::ensure_can_fit(iter)
    }
}

/// A marker for both a minimum and maximum capacity constraint.
///
/// If `MIN == MAX`, then consider using [`ExactSize`] instead.
///
/// # Type Parameters
///
/// * `MIN`: The minimum size of the constraint.
/// * `MAX`: The maximum size of the constraint.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct MinMaxCap<const MIN: usize, const MAX: usize> {}

impl<const MIN: usize, const MAX: usize> MinCap for MinMaxCap<MIN, MAX> {
    const MIN_CAP: usize = MIN;
}

impl<const MIN: usize, const MAX: usize> MaxCap for MinMaxCap<MIN, MAX> {
    const MAX_CAP: usize = MAX;
}

impl<const MIN: usize, const MAX: usize> CapConstraint for MinMaxCap<MIN, MAX> {
    type Error = CapError<Self>;

    fn check_if_can_fit<I: Iterator + ?Sized>(iter: &I) -> Result<(), Self::Error> {
        CapError::ensure_can_fit(iter)
    }
}

/// A marker for an exact size constraint, where `MIN == MAX`.
///
/// # Type Parameters
///
/// * `SIZE`: The size of both the minimum and maximum capacity constraints.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct ExactSize<const SIZE: usize> {}

impl<const SIZE: usize> MinCap for ExactSize<SIZE> {
    const MIN_CAP: usize = SIZE;
}

impl<const SIZE: usize> MaxCap for ExactSize<SIZE> {
    const MAX_CAP: usize = SIZE;
}

impl<const SIZE: usize> CapConstraint for ExactSize<SIZE> {
    type Error = CapError<Self>;

    fn check_if_can_fit<I: Iterator + ?Sized>(iter: &I) -> Result<(), Self::Error> {
        CapError::ensure_can_fit(iter)
    }
}
