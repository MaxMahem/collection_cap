use crate::StaticCap;
use crate::cap::ExactCapVal;

impl<const N: usize, T> StaticCap for [T; N] {
    type Cap = ExactCapVal;
    const CAP: Self::Cap = ExactCapVal(N);
}
