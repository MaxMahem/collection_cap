#![allow(dead_code)]

use std::ops::Range;

use collection_cap::err::{Overflows, Underflows};
use size_hinter::InvalidIterator;

pub const CAP: usize = 10;

pub const FITS_ITER: Range<i32> = 0..(CAP as i32);
pub const OVER_ITER: Range<i32> = 0..((CAP + 1) as i32);
pub const UNDER_ITER: Range<i32> = 0..((CAP - 1) as i32);

pub const CAP_OVERFLOW: Overflows = Overflows::new(CAP + 1, CAP);
pub const CAP_UNDERFLOW: Underflows = Underflows::new(CAP - 1, CAP);

pub const INVALID_ITERATOR: InvalidIterator<i32> = InvalidIterator::DEFAULT;
