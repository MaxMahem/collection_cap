use core::ops::Bound::*;
use core::ops::RangeBounds;

use collection_cap::IterCapExt;
use collection_cap::cap::{StaticExactCap, StaticMaxCap, StaticMinCap, StaticMinMaxCap, StaticUnboundedCap};
use collection_cap::err::{StaticCapError, StaticCapOverflow, StaticCapUnderflow};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

mod min_cap_marker {
    use super::*;

    check_eq!(compatible: COMPAT_ITER.ensure_compatible::<StaticMinCap<CAP>>() => Ok(()));
    check_eq!(underflow: UNDER_ITER.ensure_compatible::<StaticMinCap<CAP>>()
            => Err(StaticCapUnderflow::<StaticMinCap<CAP>>::new(UNDER_CAP)));

    panics!(bad_iter: INVALID_ITER.ensure_compatible::<StaticMinCap<CAP>>()
            => "Invalid size hint");

    mod range_bounds {
        use super::*;

        check_eq!(start_bound: StaticMinCap::<CAP>.start_bound() => Included(&CAP));
        check_eq!(end_bound: StaticMinCap::<CAP>.end_bound() => Unbounded);
    }
}

mod max_cap_marker {
    use super::*;

    check_eq!(compatible: COMPAT_ITER.ensure_compatible::<StaticMaxCap<CAP>>() => Ok(()));
    check_eq!(overflow: OVER_ITER.ensure_compatible::<StaticMaxCap<CAP>>()
            => Err(StaticCapOverflow::<StaticMaxCap<CAP>>::new(OVER_CAP)));

    panics!(bad_iter: INVALID_ITER.ensure_compatible::<StaticMaxCap<CAP>>()
            => "Invalid size hint");

    mod range_bounds {
        use super::*;

        check_eq!(start_bound: StaticMaxCap::<CAP>.start_bound() => Unbounded);
        check_eq!(end_bound: StaticMaxCap::<CAP>.end_bound() => Included(&CAP));
    }
}

mod min_max_cap {
    use super::*;

    check_eq!(compatible: COMPAT_ITER.ensure_compatible::<StaticMinMaxCap<CAP, CAP>>() => Ok(()));
    check_eq!(overflow: OVER_ITER.ensure_compatible::<StaticMinMaxCap<CAP, CAP>>()
            => Err(StaticCapError::Overflow(StaticCapOverflow::new(OVER_CAP))));
    check_eq!(underflow: UNDER_ITER.ensure_compatible::<StaticMinMaxCap<CAP, CAP>>()
            => Err(StaticCapError::Underflow(StaticCapUnderflow::new(UNDER_CAP))));

    panics!(bad_iter: INVALID_ITER.ensure_compatible::<StaticMinMaxCap<CAP, CAP>>()
            => "Invalid size hint");

    mod range_bounds {
        use super::*;

        check_eq!(start_bound: StaticMinMaxCap::<CAP, CAP>.start_bound() => Included(&CAP));
        check_eq!(end_bound: StaticMinMaxCap::<CAP, CAP>.end_bound() => Included(&CAP));
    }
}

mod exact_size {
    use super::*;

    check_eq!(compatible: COMPAT_ITER.ensure_compatible::<StaticExactCap<CAP>>() => Ok(()));
    check_eq!(overflow: OVER_ITER.ensure_compatible::<StaticExactCap<CAP>>()
            => Err(StaticCapError::Overflow(StaticCapOverflow::new(OVER_CAP))));
    check_eq!(underflow: UNDER_ITER.ensure_compatible::<StaticExactCap<CAP>>()
            => Err(StaticCapError::Underflow(StaticCapUnderflow::new(UNDER_CAP))));

    panics!(bad_iter: INVALID_ITER.ensure_compatible::<StaticExactCap<CAP>>()
            => "Invalid size hint");

    mod range_bounds {
        use super::*;

        check_eq!(start_bound: StaticExactCap::<CAP>.start_bound() => Included(&CAP));
        check_eq!(end_bound: StaticExactCap::<CAP>.end_bound() => Included(&CAP));
    }
}

mod unbounded {
    use super::*;

    check_eq!(compatible: COMPAT_ITER.ensure_compatible::<StaticUnboundedCap>() => Ok(()));

    mod range_bounds {
        use super::*;

        check_eq!(start_bound: StaticUnboundedCap.start_bound() => Unbounded);
        check_eq!(end_bound: StaticUnboundedCap.end_bound() => Unbounded);
    }
}
