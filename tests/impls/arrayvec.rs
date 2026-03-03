use collection_cap::err::MinOverflow;
use collection_cap::{IterCapExt, VariableCap};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

use arrayvec::ArrayVec;

type TestArrayVec = ArrayVec<i32, CAP>;

mod variable_cap {
    use super::*;

    check_eq!(capacity: VariableCap::capacity(&TestArrayVec::new()) => MAX_CAP_VAL);

    check_eq!(compatible: COMPAT_ITER.ensure_compatible_with(&TestArrayVec::new()) => Ok(()));
    check_eq!(overflow: OVER_ITER.ensure_compatible_with(&TestArrayVec::new()) 
        => Err(MIN_OVERFLOWS));
}

mod static_cap {
    use super::*;

    check_eq!(compatible: COMPAT_ITER.ensure_compatible::<TestArrayVec>() => Ok(()));
    check_eq!(overflow: OVER_ITER.ensure_compatible::<TestArrayVec>()
        => Err(MinOverflow::new_static(OVER_CAP)));
}

panics!(bad_iter: INVALID_ITER.ensure_compatible::<TestArrayVec>()
    => "Invalid size hint");
