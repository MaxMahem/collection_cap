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

        check_eq!(new: MinUnderflow::<MinCapVal>::new(UNDER_CAP, MIN_CAP_VAL) => MIN_UNDERFLOWS);
        panics!(panic_new: MinUnderflow::<MinCapVal>::new(CAP, MIN_CAP_VAL) => "min_size must be < min_cap");
        check_eq!(min_size: MIN_UNDERFLOWS.min_size() => UNDER_CAP);
        check_eq!(min_cap: MIN_UNDERFLOWS.min_cap() => &MIN_CAP_VAL);
    }

    mod static_cap {
        use super::*;

        check_eq!(new: MinUnderflow::<MinMaxCap>::new(UNDER_CAP) => MinUnderflow::<MinMaxCap>::new(UNDER_CAP));
        panics!(panic_new: MinUnderflow::<MinMaxCap>::new(CAP) => "min_size must be < C::MIN_CAP");
        check_eq!(min_size: MinUnderflow::<MinMaxCap>::new(UNDER_CAP).min_size() => UNDER_CAP);
        check_eq!(min_cap: *MinUnderflow::<MinMaxCap>::new(UNDER_CAP).min_cap() => MinMaxCap::CAP);
    }
}

mod overflows {
    use super::*;

    mod dynamic {
        use super::*;

        check_eq!(fixed: MaxOverflow::<MaxCapVal>::fixed(OVER_CAP, MAX_CAP_VAL) => MAX_OVERFLOWS);
        panics!(panic_fixed: MaxOverflow::<MaxCapVal>::fixed(CAP, MAX_CAP_VAL) => "max_size must be > max_cap");
        check_eq!(unbounded: MaxOverflow::<MaxCapVal>::unbounded(MAX_CAP_VAL) => MAX_OVERFLOWS_UNBOUNDED);
        check_eq!(max_size: MAX_OVERFLOWS.max_size() => FIXED_UPPER_BOUND);
        check_eq!(max_cap: *MAX_OVERFLOWS.max_cap() => MAX_CAP_VAL);
    }

    mod static_cap {
        use super::*;

        check_eq!(fixed: MaxOverflow::<MinMaxCap>::fixed(OVER_CAP) => MaxOverflow::<MinMaxCap>::fixed(OVER_CAP));
        panics!(panic_fixed: MaxOverflow::<MinMaxCap>::fixed(CAP) => "max_size must be > C::MAX_CAP");
        check_eq!(unbounded: MaxOverflow::<MinMaxCap>::UNBOUNDED.max_size() => UpperBound::Unbounded);
        check_eq!(fixed_max_size: MaxOverflow::<MinMaxCap>::fixed(OVER_CAP).max_size() => FIXED_UPPER_BOUND);
        check_eq!(max_cap: *MaxOverflow::<MinMaxCap>::fixed(OVER_CAP).max_cap() => MinMaxCap::CAP);
    }
}

mod spans {
    use super::*;

    check_eq!(new: FitErrorSpan::new(MAX_OVERFLOWS, MIN_UNDERFLOWS) => FIT_ERROR_SPAN);
    check_eq!(overflow: FIT_ERROR_SPAN.overflow() => &MAX_OVERFLOWS);
    check_eq!(underflow: FIT_ERROR_SPAN.underflow() => &MIN_UNDERFLOWS);

    panics!(panic_new: FitErrorSpan::new(
        MaxOverflow::<MaxCapVal>::fixed(10, MaxCapVal(5)),
        MinUnderflow::<MinCapVal>::new(20, MinCapVal(25))
    ) => "underflow and overflow must intersect");
}

mod errors {
    use super::*;

    mod dynamic {
        use super::*;

        check_eq!(from_overflow: FitError::from(MAX_OVERFLOWS) => FIT_ERROR_OVERFLOW);
        check_eq!(from_underflow: FitError::from(MIN_UNDERFLOWS) => FIT_ERROR_UNDERFLOW);
        check_eq!(from_both: FitError::from(FIT_ERROR_SPAN) => FIT_ERROR_BOTH);
    }

    mod static_cap {
        use super::*;

        check_eq!(from_overflow: MaxOverflow::<MaxCap>::fixed(OVER_CAP)
            .pipe(FitError::<MinCap, MaxCap>::from) 
            => MaxOverflow::<MaxCap>::fixed(OVER_CAP).pipe(FitError::Overflow));
        check_eq!(from_underflow: MinUnderflow::<MinCap>::new(UNDER_CAP)
            .pipe(FitError::<MinCap, MaxCap>::from) 
            => MinUnderflow::<MinCap>::new(UNDER_CAP).pipe(FitError::Underflow));
        check_eq!(from_both: FitErrorSpan::new(
            MaxOverflow::<MaxCap>::fixed(OVER_CAP),
            MinUnderflow::<MinCap>::new(UNDER_CAP)
        ).pipe(FitError::from) => FitErrorSpan::new(
            MaxOverflow::<MaxCap>::fixed(OVER_CAP),
            MinUnderflow::<MinCap>::new(UNDER_CAP)
        ).pipe(FitError::Both));
    }
}
