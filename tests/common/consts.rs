#![allow(dead_code)]

use std::ops::Range;

use collection_cap::err::{Overflows, Underflows, VarCapError};
use size_hinter::InvalidIterator;

pub const CAP: usize = 10;

pub const COMPAT_ITER: Range<i32> = 0..(CAP as i32);
pub const OVER_ITER: Range<i32> = 0..((CAP + 1) as i32);
pub const UNDER_ITER: Range<i32> = 0..((CAP - 1) as i32);

pub const CAP_OVERFLOWS: Overflows = Overflows::new(CAP + 1, CAP);
pub const CAP_UNDERFLOWS: Underflows = Underflows::new(CAP - 1, CAP);
pub const COMPAT_ERROR_OVERFLOWS: VarCapError = VarCapError::Overflows(CAP_OVERFLOWS);
pub const COMPAT_ERROR_UNDERFLOWS: VarCapError = VarCapError::Underflows(CAP_UNDERFLOWS);

pub const INVALID_ITERATOR: InvalidIterator<i32> = InvalidIterator::DEFAULT;
