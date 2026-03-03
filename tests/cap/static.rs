use core::ops::Bound::*;
use core::ops::RangeBounds;

use collection_cap::Capacity;
use collection_cap::cap::{StaticExactCap, StaticMaxCap, StaticMinCap, StaticMinMaxCap, StaticUnboundedCap};
use collection_cap::err::{
    CompatError, FitBoth, FitError, FitOverflow, FitUnderflow, MaxUnderflow, MinOverflow, StaticCapError,
};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

mod min_cap {
    use super::*;

    mod check_compat {
        use super::*;

        check_eq!(compatible: StaticMinCap::<CAP>.check_compatibility(&COMPAT_ITER) => Ok(()));
        check_eq!(underflow: StaticMinCap::<CAP>.check_compatibility(&UNDER_ITER)
            => Err(MaxUnderflow::<StaticMinCap<CAP>>::new(UNDER_CAP)));
        panics!(bad_iter: StaticMinCap::<CAP>.check_compatibility(&INVALID_ITER)
            => "Invalid size hint");
    }

    mod check_fit {
        use super::*;

        check_eq!(compatible: StaticMinCap::<CAP>.check_fit(&COMPAT_ITER) => Ok(()));
        check_eq!(underflow: StaticMinCap::<CAP>.check_fit(&UNDER_ITER)
            => Err(FitUnderflow::<StaticMinCap<CAP>>::new(UNDER_CAP)));
        panics!(bad_iter: StaticMinCap::<CAP>.check_fit(&INVALID_ITER)
            => "Invalid size hint");
    }

    mod range_bounds {
        use super::*;

        check_eq!(start_bound: StaticMinCap::<CAP>.start_bound() => Included(&CAP));
        check_eq!(end_bound: StaticMinCap::<CAP>.end_bound() => Unbounded);
    }
}

mod max_cap {
    use super::*;

    mod check_compat {
        use super::*;

        check_eq!(compatible: StaticMaxCap::<CAP>.check_compatibility(&COMPAT_ITER) => Ok(()));
        check_eq!(overflow: StaticMaxCap::<CAP>.check_compatibility(&OVER_ITER)
                => Err(MinOverflow::<StaticMaxCap<CAP>>::new(OVER_CAP)));

        panics!(bad_iter: StaticMaxCap::<CAP>.check_compatibility(&INVALID_ITER)
            => "Invalid size hint");
    }

    mod check_fit {
        use super::*;

        check_eq!(compatible: StaticMaxCap::<CAP>.check_fit(&COMPAT_ITER) => Ok(()));
        check_eq!(overflow: StaticMaxCap::<CAP>.check_fit(&OVER_ITER)
                => Err(FitOverflow::<StaticMaxCap<CAP>>::fixed(OVER_CAP)));
        check_eq!(overflow_unbounded: StaticMaxCap::<CAP>.check_fit(&OVER_ITER_UNBOUNDED)
                => Err(FitOverflow::UNBOUNDED));

        panics!(bad_iter: StaticMaxCap::<CAP>.check_fit(&INVALID_ITER)
            => "Invalid size hint");
    }

    mod range_bounds {
        use super::*;

        check_eq!(start_bound: StaticMaxCap::<CAP>.start_bound() => Unbounded);
        check_eq!(end_bound: StaticMaxCap::<CAP>.end_bound() => Included(&CAP));
    }
}

mod min_max_cap {
    use super::*;

    mod check_compat {
        use super::*;

        check_eq!(compatible: StaticMinMaxCap::<CAP, CAP>.check_compatibility(&COMPAT_ITER) => Ok(()));
        check_eq!(overflow: StaticMinMaxCap::<CAP, CAP>.check_compatibility(&OVER_ITER)
            => Err(CompatError::Overflow(MinOverflow::<StaticMaxCap<CAP>>::new(OVER_CAP))));
        check_eq!(underflow: StaticMinMaxCap::<CAP, CAP>.check_compatibility(&UNDER_ITER)
            => Err(CompatError::Underflow(MaxUnderflow::<StaticMinCap<CAP>>::new(UNDER_CAP))));

        panics!(bad_iter: StaticMinMaxCap::<CAP, CAP>.check_compatibility(&INVALID_ITER)
            => "Invalid size hint");
    }

    mod check_fit {
        use super::*;

        check_eq!(compatible: StaticMinMaxCap::<CAP, CAP>.check_fit(&COMPAT_ITER) => Ok(()));

        check_eq!(underflow: StaticMinMaxCap::<CAP, CAP>.check_fit(&UNDER_ITER)
            => Err(FitError::Underflow(FitUnderflow::<StaticMinCap<CAP>>::new(UNDER_CAP))));
        check_eq!(overflow: StaticMinMaxCap::<CAP, CAP>.check_fit(&OVER_ITER)
            => Err(FitError::Overflow(FitOverflow::<StaticMaxCap<CAP>>::fixed(OVER_CAP))));
        check_eq!(overflow_unbounded: StaticMinMaxCap::<CAP, CAP>.check_fit(&OVER_ITER_UNBOUNDED)
            => Err(FitError::Overflow(FitOverflow::UNBOUNDED)));
        check_eq!(both: StaticMinMaxCap::<CAP, CAP>.check_fit(&BOTH_ITER)
        => Err(FitError::Both(FitBoth::new(
            FitOverflow::<StaticMaxCap<CAP>>::fixed(OVER_CAP),
            FitUnderflow::<StaticMinCap<CAP>>::new(UNDER_CAP)
        ))));

        panics!(bad_iter: StaticMinMaxCap::<CAP, CAP>.check_fit(&INVALID_ITER)
            => "Invalid size hint");
    }

    mod range_bounds {
        use super::*;

        check_eq!(start_bound: StaticMinMaxCap::<CAP, CAP>.start_bound() => Included(&CAP));
        check_eq!(end_bound: StaticMinMaxCap::<CAP, CAP>.end_bound() => Included(&CAP));
    }
}

mod exact_cap {
    use super::*;

    mod check_compat {
        use super::*;

        check_eq!(compatible: StaticExactCap::<CAP>.check_compatibility(&COMPAT_ITER) => Ok(()));
        check_eq!(underflow: StaticExactCap::<CAP>.check_compatibility(&UNDER_ITER)
            => Err(StaticCapError::<StaticExactCap<CAP>>::Underflow(MaxUnderflow::<StaticMinCap<CAP>>::new(UNDER_CAP))));
        check_eq!(overflow: StaticExactCap::<CAP>.check_compatibility(&OVER_ITER)
            => Err(StaticCapError::<StaticExactCap<CAP>>::Overflow(MinOverflow::<StaticMaxCap<CAP>>::new(OVER_CAP))));

        panics!(bad_iter: StaticExactCap::<CAP>.check_compatibility(&INVALID_ITER)
            => "Invalid size hint");
    }

    mod check_fit {
        use super::*;

        check_eq!(compatible: StaticExactCap::<CAP>.check_fit(&COMPAT_ITER) => Ok(()));
        check_eq!(underflow: StaticExactCap::<CAP>.check_fit(&UNDER_ITER)
            => Err(FitError::Underflow(FitUnderflow::<StaticMinCap<CAP>>::new(UNDER_CAP))));
        check_eq!(overflow: StaticExactCap::<CAP>.check_fit(&OVER_ITER)
            => Err(FitError::Overflow(FitOverflow::<StaticMaxCap<CAP>>::fixed(OVER_CAP))));
        check_eq!(overflow_unbounded: StaticExactCap::<CAP>.check_fit(&OVER_ITER_UNBOUNDED)
            => Err(FitError::Overflow(FitOverflow::UNBOUNDED)));

        check_eq!(both: StaticExactCap::<CAP>.check_fit(&BOTH_ITER)
        => Err(FitError::Both(FitBoth::new(
            FitOverflow::<StaticMaxCap<CAP>>::fixed(OVER_CAP),
            FitUnderflow::<StaticMinCap<CAP>>::new(UNDER_CAP),
        ))));

        panics!(bad_iter: StaticExactCap::<CAP>.check_fit(&INVALID_ITER)
            => "Invalid size hint");
    }

    mod range_bounds {
        use super::*;

        check_eq!(start_bound: StaticExactCap::<CAP>.start_bound() => Included(&CAP));
        check_eq!(end_bound: StaticExactCap::<CAP>.end_bound() => Included(&CAP));
    }
}

mod unbounded {
    use super::*;

    check_eq!(compatible: StaticUnboundedCap.check_compatibility(&COMPAT_ITER) => Ok(()));
    check_eq!(fit: StaticUnboundedCap.check_fit(&COMPAT_ITER) => Ok(()));

    mod range_bounds {
        use super::*;

        check_eq!(start_bound: StaticUnboundedCap.start_bound() => Unbounded);
        check_eq!(end_bound: StaticUnboundedCap.end_bound() => Unbounded);
    }
}
