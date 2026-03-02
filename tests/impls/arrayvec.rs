use collection_cap::cap::StaticMaxCap;
use collection_cap::err::StaticCapOverflow;
use collection_cap::{IterCapExt, VariableCap};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

use arrayvec::ArrayVec;

type TestArrayVec = ArrayVec<i32, CAP>;

mod variable_cap {
    use super::*;

    check_eq!(capacity: VariableCap::capacity(&TestArrayVec::new()) => MAX_CAP);

    check_eq!(compatible: COMPAT_ITER.ensure_compatible_with(&TestArrayVec::new()) => Ok(()));
    check_eq!(overflow: OVER_ITER.ensure_compatible_with(&TestArrayVec::new()) 
        => Err(CAP_OVERFLOWS));
}

mod static_cap {
    use super::*;

    check_eq!(compatible: COMPAT_ITER.ensure_compatible::<TestArrayVec>() => Ok(()));
    check_eq!(overflow: OVER_ITER.ensure_compatible::<TestArrayVec>() 
        => Err(StaticCapOverflow::<StaticMaxCap<CAP>>::new(OVER_CAP)));
}

panics!(bad_iter: INVALID_ITER.ensure_compatible::<TestArrayVec>()
    => "Invalid size hint");
