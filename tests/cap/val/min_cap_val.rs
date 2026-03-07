pub use core::ops::Bound;

use crate::common::consts::*;

use collection_cap::cap::{MinCapVal, StaticMinCap, UnboundedCap};
use collection_cap::err::{MaxUnderflow, MinUnderflow};

use crate::common::check_eq;
use crate::{caps, check_compat, check_fit, contains_size, range_bounds};

const MIN_CAP_VAL: MinCapVal = MinCapVal(CAP);

check_eq!(capacity: MIN_CAP_VAL.capacity() => MIN_CAP_VAL);
check_eq!(from_static: MinCapVal::from(StaticMinCap::<CAP>) => MIN_CAP_VAL);
check_eq!(from_range_from: MinCapVal::from(CAP..) => MIN_CAP_VAL);

caps!(MIN_CAP_VAL => { min: MIN_CAP_VAL, max: UnboundedCap });

contains_size!(MIN_CAP_VAL => { cap: true, under: false, over: true });

range_bounds!(MIN_CAP_VAL => { start: Bound::Included(&CAP), end: Bound::Unbounded });

const MAX_UNDERFLOW: MaxUnderflow<MinCapVal> = MaxUnderflow::<MinCapVal>::new(UNDER_CAP, MIN_CAP_VAL);
check_compat!(MIN_CAP_VAL => { overflow: Ok(()), underflow: Err(MAX_UNDERFLOW) });

const MIN_UNDERFLOW: MinUnderflow<MinCapVal> = MinUnderflow::<MinCapVal>::new(UNDER_CAP, MIN_CAP_VAL);
check_fit!(MIN_CAP_VAL => {
    underflow: Err(MIN_UNDERFLOW),
    overflow: Ok(()),
    unbounded: Ok(()),
    both: Err(MIN_UNDERFLOW)
});
