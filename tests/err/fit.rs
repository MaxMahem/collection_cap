use tap::Pipe;

use collection_cap::StaticCap;
use collection_cap::cap::{MaxCapVal, MinCapVal};
use collection_cap::err::{FitError, FitErrorSpan, MaxOverflow, MinUnderflow, UpperBound};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

mod underflows {
    use super::*;

    mod dynamic {
        use super::*;

        check_eq!(new: MinUnderflow::<MinCapVal>::new(base::UNDER_CAP, val::MIN_CAP_VAL) => err_val_fit::MIN_UNDERFLOWS);
        panics!(panic_new: MinUnderflow::<MinCapVal>::new(base::CAP, val::MIN_CAP_VAL) => "min_size must be < min_cap");
        check_eq!(min_size: err_val_fit::MIN_UNDERFLOWS.min_size() => base::UNDER_CAP);
        check_eq!(min_cap: err_val_fit::MIN_UNDERFLOWS.min_cap() => &val::MIN_CAP_VAL);
    }

    mod static_cap {
        use super::*;

        check_eq!(new: MinUnderflow::<stat::MinCap>::new(base::UNDER_CAP) => MinUnderflow::<stat::MinCap>::new(base::UNDER_CAP));
        panics!(panic_new: MinUnderflow::<stat::MinCap>::new(base::CAP) => "min_size must be < MIN");
        check_eq!(min_size: MinUnderflow::<stat::MinCap>::new(base::UNDER_CAP).min_size() => base::UNDER_CAP);
        check_eq!(min_cap: *MinUnderflow::<stat::MinCap>::new(base::UNDER_CAP).min_cap() => stat::MinCap::CAP);
    }
}

mod overflows {
    use super::*;

    mod dynamic {
        use super::*;

        check_eq!(fixed: MaxOverflow::<MaxCapVal>::fixed(base::OVER_CAP, val::MAX_CAP_VAL) => err_val_fit::MAX_OVERFLOWS);
        panics!(panic_fixed: MaxOverflow::<MaxCapVal>::fixed(base::CAP, val::MAX_CAP_VAL) => "max_size must be > max_cap");
        check_eq!(unbounded: MaxOverflow::<MaxCapVal>::unbounded(val::MAX_CAP_VAL) => err_val_fit::MAX_OVERFLOWS_UNBOUNDED);
        check_eq!(max_size: err_val_fit::MAX_OVERFLOWS.max_size() => base::FIXED_UPPER_BOUND);
        check_eq!(max_cap: *err_val_fit::MAX_OVERFLOWS.max_cap() => val::MAX_CAP_VAL);
    }

    mod static_cap {
        use super::*;

        check_eq!(fixed: MaxOverflow::<stat::MaxCap>::fixed(base::OVER_CAP) => MaxOverflow::<stat::MaxCap>::fixed(base::OVER_CAP));
        panics!(panic_fixed: MaxOverflow::<stat::MaxCap>::fixed(base::CAP) => "max_size must be > MAX");
        check_eq!(unbounded: MaxOverflow::<stat::MaxCap>::UNBOUNDED.max_size() => UpperBound::Unbounded);
        check_eq!(fixed_max_size: MaxOverflow::<stat::MaxCap>::fixed(base::OVER_CAP).max_size() => base::FIXED_UPPER_BOUND);
        check_eq!(max_cap: *MaxOverflow::<stat::MaxCap>::fixed(base::OVER_CAP).max_cap() => stat::MaxCap::CAP);
    }
}

mod spans {
    use super::*;

    check_eq!(new: FitErrorSpan::new(err_val_fit::MAX_OVERFLOWS, err_val_fit::MIN_UNDERFLOWS) => err_val_fit::SPAN);
    check_eq!(overflow: err_val_fit::SPAN.overflow() => &err_val_fit::MAX_OVERFLOWS);
    check_eq!(underflow: err_val_fit::SPAN.underflow() => &err_val_fit::MIN_UNDERFLOWS);

    panics!(panic_new: FitErrorSpan::new(
        MaxOverflow::<MaxCapVal>::fixed(10, MaxCapVal(5)),
        MinUnderflow::<MinCapVal>::new(20, MinCapVal(25))
    ) => "underflow and overflow must intersect");
}

mod errors {
    use super::*;

    mod dynamic {
        use super::*;

        check_eq!(from_overflow: FitError::from(err_val_fit::MAX_OVERFLOWS) => err_val_fit::OVERFLOW);
        check_eq!(from_underflow: FitError::from(err_val_fit::MIN_UNDERFLOWS) => err_val_fit::UNDERFLOW);
        check_eq!(from_both: FitError::from(err_val_fit::SPAN) => err_val_fit::BOTH);
    }

    mod static_cap {
        use super::*;

        check_eq!(from_overflow: MaxOverflow::<stat::MaxCap>::fixed(base::OVER_CAP)
            .pipe(FitError::<stat::MinCap, stat::MaxCap>::from) 
            => MaxOverflow::<stat::MaxCap>::fixed(base::OVER_CAP).pipe(FitError::Overflow));
        check_eq!(from_underflow: MinUnderflow::<stat::MinCap>::new(base::UNDER_CAP)
            .pipe(FitError::<stat::MinCap, stat::MaxCap>::from) 
            => MinUnderflow::<stat::MinCap>::new(base::UNDER_CAP).pipe(FitError::Underflow));
        check_eq!(from_both: FitErrorSpan::new(
            MaxOverflow::<stat::MaxCap>::fixed(base::OVER_CAP),
            MinUnderflow::<stat::MinCap>::new(base::UNDER_CAP)
        ).pipe(FitError::from) => FitErrorSpan::new(
            MaxOverflow::<stat::MaxCap>::fixed(base::OVER_CAP),
            MinUnderflow::<stat::MinCap>::new(base::UNDER_CAP)
        ).pipe(FitError::Both));
    }
}
