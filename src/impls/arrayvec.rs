use arrayvec::ArrayVec;
use tap::Pipe;

use crate::cap::{MaxCapVal, MinCapVal, StaticMaxCap};
use crate::{ConstMaxCap, ConstMinCap, StaticCap, VariableCap};

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

impl<T, const N: usize> ConstMaxCap for ArrayVec<T, N> {
    const MAX_CAP: MaxCapVal = MaxCapVal(N);
}

impl<T, const N: usize> ConstMinCap for ArrayVec<T, N> {
    const MIN_CAP: MinCapVal = MinCapVal(0);
}
