use collection_cap::IterCapExt;
use collection_cap::err::{StaticCapError, StaticCapOverflow, StaticCapUnderflow};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

type TestArray = [i32; CAP];

check_eq!(cap_constraint: COMPAT_ITER.ensure_compatible::<TestArray>() => Ok(()));
check_eq!(cap_constraint_overflow: OVER_ITER.ensure_compatible::<TestArray>() 
    => Err(StaticCapError::Overflow(StaticCapOverflow::new(OVER_CAP))));
check_eq!(cap_constraint_underflow: UNDER_ITER.ensure_compatible::<TestArray>() 
    => Err(StaticCapError::Underflow(StaticCapUnderflow::new(UNDER_CAP))));

panics!(bad_iter: INVALID_ITER.ensure_compatible::<TestArray>() 
    => "Invalid size hint");
