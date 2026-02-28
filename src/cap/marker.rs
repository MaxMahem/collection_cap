use crate::StaticCap;
use crate::cap::{ExactCapVal, MaxCapVal, MinCapVal, MinMaxCapVal};

/// A marker for a minimum capacity constraint.
///
/// # Type Parameters
///
/// * `MIN`: The minimum size of the constraint.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct MinCapMarker<const MIN: usize> {}

impl<const MIN: usize> StaticCap for MinCapMarker<MIN> {
    type Cap = MinCapVal;
    const CAP: Self::Cap = MinCapVal(MIN);
}

/// A marker for a maximum capacity constraint.
///
/// # Type Parameters
///
/// * `MAX`: The maximum size of the constraint.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct MaxCapMarker<const MAX: usize> {}

impl<const MAX: usize> StaticCap for MaxCapMarker<MAX> {
    type Cap = MaxCapVal;
    const CAP: Self::Cap = MaxCapVal(MAX);
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

impl<const MIN: usize, const MAX: usize> StaticCap for MinMaxCap<MIN, MAX> {
    type Cap = MinMaxCapVal;
    const CAP: Self::Cap = MinMaxCapVal::new(MIN, MAX);
}

/// A marker for an exact size constraint, where `MIN == MAX`.
///
/// # Type Parameters
///
/// * `SIZE`: The size of both the minimum and maximum capacity constraints.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct ExactSize<const SIZE: usize> {}

impl<const SIZE: usize> StaticCap for ExactSize<SIZE> {
    type Cap = ExactCapVal;
    const CAP: Self::Cap = ExactCapVal(SIZE);
}
