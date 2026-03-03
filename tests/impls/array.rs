use collection_cap::IterCapExt;
use collection_cap::cap::{StaticMaxCap, StaticMinCap};
use collection_cap::err::{CompatError, MaxUnderflow, MinOverflow};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

type TestArray = [i32; CAP];

check_eq!(cap_constraint: COMPAT_ITER.ensure_compatible::<TestArray>() => Ok(()));

check_eq!(cap_constraint_overflow: OVER_ITER.ensure_compatible::<TestArray>() 
    => Err(CompatError::Overflow(MinOverflow::<StaticMaxCap<CAP>>::new(OVER_CAP))));

check_eq!(cap_constraint_underflow: UNDER_ITER.ensure_compatible::<TestArray>() 
    => Err(CompatError::Underflow(MaxUnderflow::<StaticMinCap<CAP>>::new(UNDER_CAP))));

panics!(bad_iter: INVALID_ITER.ensure_compatible::<TestArray>()
    => "Invalid size hint");
