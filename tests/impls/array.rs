use collection_cap::IterCapExt;
use collection_cap::err::CapError;

use crate::common::consts::*;
use crate::common::{check_eq, panics};

type TestArray = [i32; CAP];

check_eq!(cap_constraint: COMPAT_ITER.ensure_compatible::<TestArray>() => Ok(()));
check_eq!(cap_constraint_overflow: OVER_ITER.ensure_compatible::<TestArray>() 
    => Err(CapError::Overflows(CAP_OVERFLOWS)));
check_eq!(cap_constraint_underflow: UNDER_ITER.ensure_compatible::<TestArray>() 
    => Err(CapError::Underflows(CAP_UNDERFLOWS)));

panics!(bad_iter: INVALID_ITER.ensure_compatible::<TestArray>() 
    => "Invalid size hint");
