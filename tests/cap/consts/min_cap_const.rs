use std::ops::{Bound, RangeFrom};

use collection_cap::cap::{ConstMinCap, UnboundedCap};

use collection_cap::err::{MaxUnderflow, MinUnderflow};

use crate::common::check_eq;
use crate::common::consts::*;
use crate::{caps, check_intersects, check_overlaps, contains_size, range_bounds};

caps!(ConstMinCap::<CAP> => { min: ConstMinCap::<CAP>, max: UnboundedCap });

contains_size!(ConstMinCap::<CAP> => { cap: true, under: false, over: true });

const MAX_UNDERFLOW: MaxUnderflow<ConstMinCap<CAP>> = MaxUnderflow::<ConstMinCap<CAP>>::new(UNDER_CAP);
check_intersects!(ConstMinCap::<CAP> => { overflow: Ok(()), underflow: Err(MAX_UNDERFLOW) });

const MIN_UNDERFLOW: MinUnderflow<ConstMinCap<CAP>> = MinUnderflow::<ConstMinCap<CAP>>::new(UNDER_CAP);
check_overlaps!(ConstMinCap::<CAP> => {
    underflow: Err(MIN_UNDERFLOW),
    overflow: Ok(()),
    unbounded: Ok(()),
    both: Err(MIN_UNDERFLOW)
});

range_bounds!(ConstMinCap::<CAP> => { start: Bound::Included(&CAP), end: Bound::Unbounded });

check_eq!(range_const: ConstMinCap::<CAP>::RANGE => CAP..);
check_eq!(from_range_from: RangeFrom::<usize>::from(ConstMinCap::<CAP>) 
    => ConstMinCap::<CAP>::RANGE);
