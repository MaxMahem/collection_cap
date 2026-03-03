use crate::StaticCap;
use crate::cap::StaticExactCap;

impl<const N: usize, T> StaticCap for [T; N] {
    type Cap = StaticExactCap<N>;

    const CAP: Self::Cap = StaticExactCap {};
}
