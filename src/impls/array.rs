use crate::err::CapError;
use crate::{StaticCap, MaxCap, MinCap};

impl<const N: usize, T> MaxCap for [T; N] {
    /// Always returns `N`.
    const MAX_CAP: usize = N;
}

impl<const N: usize, T> MinCap for [T; N] {
    /// Always returns `N`.
    const MIN_CAP: usize = N;
}

impl<const N: usize, T> StaticCap for [T; N] {
    type Error = CapError<Self>;

    fn check_compatability<I: Iterator + ?Sized>(iter: &I) -> Result<(), Self::Error> {
        CapError::ensure_compatible(iter)
    }
}
