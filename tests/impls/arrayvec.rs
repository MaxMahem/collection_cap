use collection_cap::IterCapExt;

use crate::common::consts::*;
use crate::common::{check_eq, panics};

use arrayvec::ArrayVec;

type TestArrayVec = ArrayVec<i32, CAP>;

check_eq!(cap_constraint: COMPAT_ITER.ensure_compatible::<TestArrayVec>() => Ok(()));
check_eq!(cap_constraint_overflow: OVER_ITER.ensure_compatible::<TestArrayVec>() 
    => Err(CAP_OVERFLOWS));

panics!(bad_iter: INVALID_ITER.ensure_compatible::<TestArrayVec>()
    => "Invalid size hint");
