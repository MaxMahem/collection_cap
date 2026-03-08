#![allow(dead_code)]

pub use collection_cap::cap::{ConstExactCap, ConstMaxCap, ConstMinCap, ConstMinMaxCap};
pub use collection_cap::cap::{ExactCapVal, MaxCapVal, MinCapVal, MinMaxCapVal, UnboundedCap};
pub use collection_cap::err::{
    IntersectError, EmptyRange, OverlapError, OverlapErrorSpan, FromRangeError, InvalidRange, MaxOverflow, MaxUnderflow,
    MinOverflow, MinUnderflow, UpperBound,
};
pub use collection_cap::{Capacity, ConstCap, VariableCap};

use std::ops::{Range, RangeFrom, RangeInclusive};

pub const CAP: usize = 10;
pub const OVER_CAP: usize = CAP + 1;
pub const UNDER_CAP: usize = CAP - 1;

pub const CAP_RANGE: RangeInclusive<usize> = CAP..=CAP;

pub mod iter {
    use super::*;
    use size_hinter::{InvalidIterator, TestIterator};

    pub const INTERSECT_ITER: Range<i32> = 0..(CAP as i32);
    pub const OVER_ITER: Range<i32> = 0..(OVER_CAP as i32);
    pub const OVER_ITER_UNBOUNDED: RangeFrom<i32> = 0..;
    pub const UNDER_ITER: Range<i32> = 0..(UNDER_CAP as i32);

    pub const BOTH_ITER: TestIterator<i32> = TestIterator::new((UNDER_CAP, Some(OVER_CAP)));
    pub const INVALID_ITER: InvalidIterator<i32> = InvalidIterator::DEFAULT;
}
