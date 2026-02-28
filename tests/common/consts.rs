#![allow(dead_code)]

use std::ops::Range;

use collection_cap::cap::{MaxCapVal, MinCapVal};
use collection_cap::err::{CapError, Overflows, Underflows};
use size_hinter::InvalidIterator;

pub const CAP: usize = 10;
pub const OVER_CAP: usize = CAP + 1;
pub const UNDER_CAP: usize = CAP - 1;

pub const COMPAT_ITER: Range<i32> = 0..(CAP as i32);
pub const OVER_ITER: Range<i32> = 0..(OVER_CAP as i32);
pub const UNDER_ITER: Range<i32> = 0..(UNDER_CAP as i32);

pub const MAX_CAP: MaxCapVal = MaxCapVal(CAP);
pub const MIN_CAP: MinCapVal = MinCapVal(CAP);

pub const CAP_OVERFLOWS: Overflows = Overflows::new(OVER_CAP, MAX_CAP);
pub const CAP_UNDERFLOWS: Underflows = Underflows::new(UNDER_CAP, MIN_CAP);
pub const CAP_ERROR_OVERFLOW: CapError = CapError::Overflows(CAP_OVERFLOWS);
pub const CAP_ERROR_UNDERFLOW: CapError = CapError::Underflows(CAP_UNDERFLOWS);

pub const INVALID_ITER: InvalidIterator<i32> = InvalidIterator::DEFAULT;
