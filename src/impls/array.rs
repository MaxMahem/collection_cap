use crate::cap::{MaxCapVal, MinCapVal, StaticExactCap};
use crate::{ConstMaxCap, ConstMinCap, StaticCap};

impl<const N: usize, T> StaticCap for [T; N] {
    type Cap = StaticExactCap<N>;
    const CAP: Self::Cap = StaticExactCap {};
}

impl<const N: usize, T> ConstMaxCap for [T; N] {
    const MAX_CAP: MaxCapVal = MaxCapVal(N);
}

impl<const N: usize, T> ConstMinCap for [T; N] {
    const MIN_CAP: MinCapVal = MinCapVal(N);
}
