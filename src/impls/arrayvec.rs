use arrayvec::ArrayVec;
use tap::Pipe;

use crate::cap::{MaxCapVal, StaticMaxCap};
use crate::{StaticCap, VariableCap};

impl<T, const N: usize> VariableCap for ArrayVec<T, N> {
    type Cap = MaxCapVal;

    fn capacity(&self) -> MaxCapVal {
        self.remaining_capacity().pipe(MaxCapVal)
    }
}

impl<T, const N: usize> StaticCap for ArrayVec<T, N> {
    type Cap = StaticMaxCap<N>;

    const CAP: Self::Cap = StaticMaxCap {};
}
