use collection_cap::IterCapExt;
use collection_cap::cap::StaticExactCap;
use collection_cap::err::{CompatError, MaxUnderflow, MinOverflow};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

type TestArray = [i32; CAP];

check_eq!(cap_constraint: COMPAT_ITER.ensure_compatible::<TestArray>() => Ok(()));

const OVER_ERROR: CompatError<StaticExactCap<CAP>, StaticExactCap<CAP>> =
    CompatError::Overflow(MinOverflow::new_static(OVER_CAP));
check_eq!(cap_constraint_overflow: OVER_ITER.ensure_compatible::<TestArray>() 
    => Err(OVER_ERROR));

const UNDER_ERROR: CompatError<StaticExactCap<CAP>, StaticExactCap<CAP>> =
    CompatError::Underflow(MaxUnderflow::new_static(UNDER_CAP));
check_eq!(cap_constraint_underflow: UNDER_ITER.ensure_compatible::<TestArray>() 
    => Err(UNDER_ERROR));

panics!(bad_iter: INVALID_ITER.ensure_compatible::<TestArray>()
    => "Invalid size hint");
