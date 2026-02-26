use arrayvec::ArrayVec;

use crate::err::TargetOverflow;
use crate::{CapConstraint, MaxCap, RemainingCap};

impl<T, const N: usize> MaxCap for ArrayVec<T, N> {
    const MAX_CAP: usize = N;
}

impl<T, const N: usize> RemainingCap for ArrayVec<T, N> {
    fn remaining_cap(&self) -> usize {
        self.remaining_capacity()
    }
}

impl<T, const N: usize> CapConstraint for ArrayVec<T, N> {
    type Error = TargetOverflow<Self>;

    fn check_if_can_fit<I: Iterator + ?Sized>(iter: &I) -> Result<(), Self::Error> {
        TargetOverflow::ensure_can_fit(iter)
    }
}
