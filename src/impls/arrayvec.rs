use arrayvec::ArrayVec;

use crate::cap::{ConstMaxCap, MaxCapVal};
use crate::{ConstCap, VariableCap};

impl<T, const N: usize> VariableCap for ArrayVec<T, N> {
    type Cap = MaxCapVal;

    fn capacity(&self) -> MaxCapVal {
        self.remaining_capacity().into()
    }
}

impl<T, const N: usize> ConstCap for ArrayVec<T, N> {
    type Cap = ConstMaxCap<N>;

    const CAP: Self::Cap = ConstMaxCap {};
}
