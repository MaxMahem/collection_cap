pub use core::ops::Bound;

use crate::common::consts::*;

use collection_cap::cap::{ConstMinCap, MinCapVal, UnboundedCap};
use collection_cap::err::{MaxUnderflow, MinUnderflow};

use crate::common::check_eq;
use crate::{caps, check_intersects, check_overlaps, contains_size, range_bounds};

const MIN_CAP_VAL: MinCapVal = MinCapVal(CAP);

check_eq!(capacity: MIN_CAP_VAL.capacity() => MIN_CAP_VAL);
check_eq!(from_static: MinCapVal::from(ConstMinCap::<CAP>) => MIN_CAP_VAL);
check_eq!(from_range_from: MinCapVal::from(CAP..) => MIN_CAP_VAL);

caps!(MIN_CAP_VAL => { min: MIN_CAP_VAL, max: UnboundedCap });

contains_size!(MIN_CAP_VAL => { cap: true, under: false, over: true });

range_bounds!(MIN_CAP_VAL => { start: Bound::Included(&CAP), end: Bound::Unbounded });

const MAX_UNDERFLOW: MaxUnderflow<MinCapVal> = MaxUnderflow::<MinCapVal>::new(UNDER_CAP, MIN_CAP_VAL);
check_intersects!(MIN_CAP_VAL => { overflow: Ok(()), underflow: Err(MAX_UNDERFLOW) });

const MIN_UNDERFLOW: MinUnderflow<MinCapVal> = MinUnderflow::<MinCapVal>::new(UNDER_CAP, MIN_CAP_VAL);
check_overlaps!(MIN_CAP_VAL => {
    underflow: Err(MIN_UNDERFLOW),
    overflow: Ok(()),
    unbounded: Ok(()),
    both: Err(MIN_UNDERFLOW)
});
