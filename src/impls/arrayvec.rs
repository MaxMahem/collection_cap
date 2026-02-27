use arrayvec::ArrayVec;

use crate::cap::MaxCapVal;
use crate::err::{CapOverflow, Overflows};
use crate::{MaxCap, StaticCap, VariableCap};

impl<T, const N: usize> MaxCap for ArrayVec<T, N> {
    const MAX_CAP: usize = N;
}

impl<T, const N: usize> VariableCap for ArrayVec<T, N> {
    type Error = Overflows;

    fn check_compatability<I: Iterator + ?Sized>(&self, iter: &I) -> Result<(), Self::Error> {
        MaxCapVal(self.remaining_capacity()).check_compatability(iter)
    }
}

impl<T, const N: usize> StaticCap for ArrayVec<T, N> {
    type Error = CapOverflow<Self>;

    fn check_compatability<I: Iterator + ?Sized>(iter: &I) -> Result<(), Self::Error> {
        CapOverflow::ensure_compatible(iter)
    }
}
