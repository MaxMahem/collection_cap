use tap::Pipe;

use collection_cap::StaticCap;
use collection_cap::cap::{MaxCapVal, MinCapVal};
use collection_cap::err::{CompatError, MaxUnderflow, MinOverflow};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

mod underflows {
    use super::*;

    mod dynamic {
        use super::*;

        check_eq!(new: MaxUnderflow::<MinCapVal>::new(UNDER_CAP, MIN_CAP_VAL) => MAX_UNDERFLOWS);
        panics!(panic_new: MaxUnderflow::<MinCapVal>::new(CAP, MIN_CAP_VAL) => "max_size must be < min_cap");
        check_eq!(max_size: MAX_UNDERFLOWS.max_size() => UNDER_CAP);
        check_eq!(min_cap: *MAX_UNDERFLOWS.min_cap() => MIN_CAP_VAL);
    }

    mod static_cap {
        use super::*;

        check_eq!(new: MaxUnderflow::<MinMaxCap>::new(UNDER_CAP) 
            => MaxUnderflow::<MinMaxCap>::new(UNDER_CAP));
        panics!(panic_new: MaxUnderflow::<MinMaxCap>::new(CAP) => "max_size must be < C::MIN_CAP");
        check_eq!(max_size: MaxUnderflow::<MinMaxCap>::new(UNDER_CAP).max_size() => UNDER_CAP);
        check_eq!(min_cap: *MaxUnderflow::<MinMaxCap>::new(UNDER_CAP).min_cap() => MinMaxCap::CAP);
    }
}

mod overflows {
    use super::*;

    mod dynamic {
        use super::*;

        check_eq!(new: MinOverflow::<MaxCapVal>::new(OVER_CAP, MAX_CAP_VAL) => MIN_OVERFLOWS);
        panics!(panic_new: MinOverflow::<MaxCapVal>::new(CAP, MAX_CAP_VAL) => "min_size must be > max_cap");
        check_eq!(min_size: MIN_OVERFLOWS.min_size() => OVER_CAP);
        check_eq!(max_cap: *MIN_OVERFLOWS.max_cap() => MAX_CAP_VAL);
    }

    mod static_cap {
        use super::*;

        check_eq!(new: MinOverflow::<MinMaxCap>::new(OVER_CAP) 
            => MinOverflow::<MinMaxCap>::new(OVER_CAP));
        panics!(panic_new: MinOverflow::<MinMaxCap>::new(CAP) => "min_size must be > C::MAX_CAP");
        check_eq!(min_size: MinOverflow::<MinMaxCap>::new(OVER_CAP).min_size() => OVER_CAP);
        check_eq!(max_cap: *MinOverflow::<MinMaxCap>::new(OVER_CAP).max_cap() => MinMaxCap::CAP);
    }
}

mod errors {
    use super::*;

    mod dynamic {
        use super::*;

        check_eq!(from_overflow: CompatError::from(MIN_OVERFLOWS) => CAP_ERROR_OVERFLOW);
        check_eq!(from_underflow: CompatError::from(MAX_UNDERFLOWS) => CAP_ERROR_UNDERFLOW);
    }

    mod static_cap {
        use super::*;

        check_eq!(from_overflow: MinOverflow::<MaxCap>::new(OVER_CAP)
            .pipe(CompatError::<MinCap, MaxCap>::from)
            => MinOverflow::<MaxCap>::new(OVER_CAP).pipe(CompatError::Overflow));
        check_eq!(from_underflow: MaxUnderflow::<MinCap>::new(UNDER_CAP)
            .pipe(CompatError::<MinCap, MaxCap>::from)
            => MaxUnderflow::<MinCap>::new(UNDER_CAP).pipe(CompatError::Underflow));
    }
}
