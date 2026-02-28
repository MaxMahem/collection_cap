use arrayvec::ArrayVec;
use tap::Pipe;

use crate::cap::MaxCapVal;
use crate::err::Overflows;
use crate::{StaticCap, Capacity};

impl<T, const N: usize> Capacity for ArrayVec<T, N> {
    type Error = Overflows;

    fn check_compatability<I: Iterator + ?Sized>(&self, iter: &I) -> Result<(), Self::Error> {
        self.remaining_capacity().pipe(MaxCapVal).check_compatability(iter)
    }
}

impl<T, const N: usize> StaticCap for ArrayVec<T, N> {
    type Cap = MaxCapVal;
    const CAP: Self::Cap = MaxCapVal(N);
}
