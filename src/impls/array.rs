use crate::cap::{MaxCapVal, MinCapVal};
use crate::err::CapError;
use crate::{MaxCap, MinCap, StaticCap};

impl<const N: usize, T> MaxCap for [T; N] {
    /// Always returns `N`.
    const MAX_CAP: MaxCapVal = MaxCapVal(N);
}

impl<const N: usize, T> MinCap for [T; N] {
    /// Always returns `N`.
    const MIN_CAP: MinCapVal = MinCapVal(N);
}

impl<const N: usize, T> StaticCap for [T; N] {
    type Error = CapError<Self>;

    fn check_compatability<I: Iterator + ?Sized>(iter: &I) -> Result<(), Self::Error> {
        CapError::ensure_compatible(iter)
    }
}
