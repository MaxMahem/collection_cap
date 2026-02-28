use core::ops::Bound::*;
use core::ops::RangeBounds;

use collection_cap::VariableCap;
use collection_cap::cap::{ExactCapVal, MaxCapVal, MinCapVal, MinMaxCapVal};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

const MIN_CAP: MinCapVal = MinCapVal(CAP);
const MAX_CAP: MaxCapVal = MaxCapVal(CAP);
const MIN_MAX_CAP: MinMaxCapVal = MinMaxCapVal::new(CAP, CAP);
const EXACT_CAP: ExactCapVal = ExactCapVal(CAP);

mod max_cap_val {
    use super::*;

    check_eq!(from_exact: MaxCapVal::from(EXACT_CAP) => MAX_CAP);

    mod range_bounds {
        use super::*;

        check_eq!(start_bound: MAX_CAP.start_bound() => Unbounded);
        check_eq!(end_bound: MAX_CAP.end_bound() => Included(&CAP));
    }

    mod check_compatibility {
        use super::*;

        check_eq!(compatible: MAX_CAP.check_compatability(&COMPAT_ITER) => Ok(()));
        check_eq!(overflow: MAX_CAP.check_compatability(&OVER_ITER)
            => Err(CAP_OVERFLOWS));

        panics!(bad_iter: MAX_CAP.check_compatability(&INVALID_ITER)
            => "Invalid size hint");
    }
}

mod min_cap_val {
    use super::*;

    check_eq!(from_exact: MinCapVal::from(EXACT_CAP) => MIN_CAP);

    mod range_bounds {
        use super::*;

        check_eq!(start_bound: MIN_CAP.start_bound() => Included(&CAP));
        check_eq!(end_bound: MIN_CAP.end_bound() => Unbounded);
    }

    mod check_compatibility {
        use super::*;

        check_eq!(compatible: MIN_CAP.check_compatability(&COMPAT_ITER) => Ok(()));
        check_eq!(underflow: MIN_CAP.check_compatability(&UNDER_ITER)
            => Err(CAP_UNDERFLOWS));

        panics!(bad_iter: MIN_CAP.check_compatability(&INVALID_ITER)
            => "Invalid size hint");
    }
}

mod min_max_cap_val {
    use std::ops::RangeBounds;

    use super::*;

    check_eq!(new: MinMaxCapVal::new(CAP, CAP) => MIN_MAX_CAP);
    check_eq!(min_val: MIN_MAX_CAP.min() => MinCapVal(CAP));
    check_eq!(max_val: MIN_MAX_CAP.max() => MaxCapVal(CAP));
    check_eq!(from_exact: MinMaxCapVal::from(EXACT_CAP) => MIN_MAX_CAP);

    mod range_bounds {
        use super::*;

        check_eq!(start_bound: MIN_MAX_CAP.start_bound() => Included(&CAP));
        check_eq!(end_bound: MIN_MAX_CAP.end_bound() => Included(&CAP));
    }

    check_eq!(eq: MIN_MAX_CAP == EXACT_CAP => true);
    check_eq!(ne: MIN_MAX_CAP != EXACT_CAP => false);

    mod check_compatibility {
        use super::*;

        check_eq!(compatible: MIN_MAX_CAP.check_compatability(&COMPAT_ITER)
            => Ok(()));
        check_eq!(overflow: MIN_MAX_CAP.check_compatability(&OVER_ITER)
            => Err(COMPAT_ERROR_OVERFLOWS));
        check_eq!(underflow: MIN_MAX_CAP.check_compatability(&UNDER_ITER)
            => Err(COMPAT_ERROR_UNDERFLOWS));

        panics!(bad_iter: MIN_MAX_CAP.check_compatability(&INVALID_ITER)
            => "Invalid size hint");

        panics!(invalid_range: MinMaxCapVal::new(CAP, CAP - 1)
            => "invalid range (start > end)");
    }
}

mod exact {
    use super::*;

    mod range_bounds {
        use super::*;

        check_eq!(start_bound: EXACT_CAP.start_bound() => Included(&CAP));
        check_eq!(end_bound: EXACT_CAP.end_bound() => Included(&CAP));
    }

    check_eq!(eq: EXACT_CAP == MIN_MAX_CAP => true);
    check_eq!(ne: EXACT_CAP != MIN_MAX_CAP => false);

    mod check_compatibility {
        use super::*;

        check_eq!(compatible: EXACT_CAP.check_compatability(&COMPAT_ITER) => Ok(()));
        check_eq!(overflow: EXACT_CAP.check_compatability(&OVER_ITER)
            => Err(COMPAT_ERROR_OVERFLOWS));
        check_eq!(underflow: EXACT_CAP.check_compatability(&UNDER_ITER)
            => Err(COMPAT_ERROR_UNDERFLOWS));

        panics!(bad_iter: EXACT_CAP.check_compatability(&INVALID_ITER)
            => "Invalid size hint");
    }
}
