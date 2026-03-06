#![allow(unused_imports)]
use crate::common::consts::*;
use super::*;

check_eq!(capacity: MIN_CAP.capacity() => MIN_CAP);
check_eq!(from_static: MinCapVal::from(StaticMinCap::<{ base::CAP }>) => MIN_CAP);
check_eq!(min_cap: MIN_CAP.min_cap() => MIN_CAP);
check_eq!(max_cap: MIN_CAP.max_cap() => UnboundedCap);
check_eq!(from_range_from: MinCapVal::from(base::CAP..) => MIN_CAP);

mod range_bounds {
    use super::*;

    check_eq!(start_bound: MIN_CAP.start_bound() => Included(&base::CAP));
    check_eq!(end_bound: MIN_CAP.end_bound() => Unbounded);
}

mod check_compat {
    use super::*;

    check_eq!(compatible: MIN_CAP.check_compatibility(&iter::COMPAT_ITER) => Ok(()));
    check_eq!(underflow: MIN_CAP.check_compatibility(&iter::UNDER_ITER) => Err(err_val_compat::MAX_UNDERFLOWS));

    panics!(bad_iter: MIN_CAP.check_compatibility(&iter::INVALID_ITER) => "Invalid size hint");
}

mod check_fit {
    use super::*;

    check_eq!(compatible: MIN_CAP.check_fit(&iter::COMPAT_ITER) => Ok(()));
    check_eq!(underflow: MIN_CAP.check_fit(&iter::UNDER_ITER) => Err(err_val_fit::MIN_UNDERFLOWS));

    panics!(bad_iter: MIN_CAP.check_fit(&iter::INVALID_ITER) => "Invalid size hint");
}
