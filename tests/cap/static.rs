use core::ops::Bound::*;
use core::ops::RangeBounds;

use collection_cap::Capacity;
use collection_cap::cap::{StaticExactCap, StaticMaxCap, StaticMinCap, StaticMinMaxCap, UnboundedCap};
use collection_cap::err::{CompatError, FitError, FitErrorSpan, MaxOverflow, MaxUnderflow, MinOverflow, MinUnderflow};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

mod min_cap {
    use super::*;

    check_eq!(min_cap: StaticMinCap::<CAP>.min_cap() => StaticMinCap::<CAP>);
    check_eq!(max_cap: StaticMinCap::<CAP>.max_cap() => UnboundedCap);

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
            => Err(MinUnderflow::<StaticMinCap<CAP>>::new(UNDER_CAP)));
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

    check_eq!(min_cap: StaticMaxCap::<CAP>.min_cap() => UnboundedCap);
    check_eq!(max_cap: StaticMaxCap::<CAP>.max_cap() => StaticMaxCap::<CAP>);

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
                => Err(MaxOverflow::<StaticMaxCap<CAP>>::fixed(OVER_CAP)));
        check_eq!(overflow_unbounded: StaticMaxCap::<CAP>.check_fit(&OVER_ITER_UNBOUNDED)
                => Err(MaxOverflow::UNBOUNDED));

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

    check_eq!(min_cap: StaticMinMaxCap::<CAP, CAP>.min_cap() => StaticMinCap::<CAP>);
    check_eq!(max_cap: StaticMinMaxCap::<CAP, CAP>.max_cap() => StaticMaxCap::<CAP>);

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
            => Err(FitError::Underflow(MinUnderflow::<StaticMinCap<CAP>>::new(UNDER_CAP))));
        check_eq!(overflow: StaticMinMaxCap::<CAP, CAP>.check_fit(&OVER_ITER)
            => Err(FitError::Overflow(MaxOverflow::<StaticMaxCap<CAP>>::fixed(OVER_CAP))));
        check_eq!(overflow_unbounded: StaticMinMaxCap::<CAP, CAP>.check_fit(&OVER_ITER_UNBOUNDED)
            => Err(FitError::Overflow(MaxOverflow::UNBOUNDED)));
        check_eq!(both: StaticMinMaxCap::<CAP, CAP>.check_fit(&BOTH_ITER)
        => Err(FitError::Both(FitErrorSpan::new(
            MaxOverflow::<StaticMaxCap<CAP>>::fixed(OVER_CAP),
            MinUnderflow::<StaticMinCap<CAP>>::new(UNDER_CAP)
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

    check_eq!(min_cap: StaticExactCap::<CAP>.min_cap() => StaticMinCap::<CAP>);
    check_eq!(max_cap: StaticExactCap::<CAP>.max_cap() => StaticMaxCap::<CAP>);

    mod check_compat {
        use super::*;

        check_eq!(compatible: StaticExactCap::<CAP>.check_compatibility(&COMPAT_ITER) => Ok(()));
        check_eq!(underflow: StaticExactCap::<CAP>.check_compatibility(&UNDER_ITER)
            => Err(CompatError::<StaticMinCap<CAP>, StaticMaxCap<CAP>>::Underflow(MaxUnderflow::<StaticMinCap<CAP>>::new(UNDER_CAP))));
        check_eq!(overflow: StaticExactCap::<CAP>.check_compatibility(&OVER_ITER)
            => Err(CompatError::<StaticMinCap<CAP>, StaticMaxCap<CAP>>::Overflow(MinOverflow::<StaticMaxCap<CAP>>::new(OVER_CAP))));

        panics!(bad_iter: StaticExactCap::<CAP>.check_compatibility(&INVALID_ITER)
            => "Invalid size hint");
    }

    mod check_fit {
        use super::*;

        check_eq!(compatible: StaticExactCap::<CAP>.check_fit(&COMPAT_ITER) => Ok(()));
        check_eq!(underflow: StaticExactCap::<CAP>.check_fit(&UNDER_ITER)
            => Err(FitError::Underflow(MinUnderflow::<StaticMinCap<CAP>>::new(UNDER_CAP))));
        check_eq!(overflow: StaticExactCap::<CAP>.check_fit(&OVER_ITER)
            => Err(FitError::Overflow(MaxOverflow::<StaticMaxCap<CAP>>::fixed(OVER_CAP))));
        check_eq!(overflow_unbounded: StaticExactCap::<CAP>.check_fit(&OVER_ITER_UNBOUNDED)
            => Err(FitError::Overflow(MaxOverflow::UNBOUNDED)));

        check_eq!(both: StaticExactCap::<CAP>.check_fit(&BOTH_ITER)
        => Err(FitError::Both(FitErrorSpan::new(
            MaxOverflow::<StaticMaxCap<CAP>>::fixed(OVER_CAP),
            MinUnderflow::<StaticMinCap<CAP>>::new(UNDER_CAP),
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
