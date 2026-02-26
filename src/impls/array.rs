use crate::err::TargetCapError;
use crate::{CapConstraint, MaxCap, MinCap};

impl<const N: usize, T> MaxCap for [T; N] {
    /// Always returns `N`.
    const MAX_CAP: usize = N;
}

impl<const N: usize, T> MinCap for [T; N] {
    /// Always returns `N`.
    const MIN_CAP: usize = N;
}

impl<const N: usize, T> CapConstraint for [T; N] {
    type Error = TargetCapError<Self>;

    fn check_if_can_fit<I: Iterator + ?Sized>(iter: &I) -> Result<(), Self::Error> {
        TargetCapError::ensure_can_fit(iter)
    }
}
