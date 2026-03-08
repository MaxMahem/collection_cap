use crate::ConstCap;
use crate::cap::ConstExactCap;

impl<const N: usize, T> ConstCap for [T; N] {
    type Cap = ConstExactCap<N>;

    const CAP: Self::Cap = ConstExactCap {};
}
