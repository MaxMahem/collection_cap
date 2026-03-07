use collection_cap::StaticCap;
use collection_cap::cap::{MaxCapVal, MinCapVal};
use collection_cap::err::{CompatError, MaxUnderflow, MinOverflow};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

const MAX_CAP_VAL: MaxCapVal = MaxCapVal(CAP);
const MIN_CAP_VAL: MinCapVal = MinCapVal(CAP);

pub const MIN_OVERFLOW: MinOverflow<MaxCapVal> = MinOverflow::<MaxCapVal>::new(OVER_CAP, MAX_CAP_VAL);
pub const MAX_UNDERFLOW: MaxUnderflow<MinCapVal> = MaxUnderflow::<MinCapVal>::new(UNDER_CAP, MIN_CAP_VAL);
pub const COMPAT_ERR_OVERFLOW: CompatError<MinCapVal, MaxCapVal> = CompatError::Overflow(MIN_OVERFLOW);
pub const COMPAT_ERR_UNDERFLOW: CompatError<MinCapVal, MaxCapVal> = CompatError::Underflow(MAX_UNDERFLOW);

pub const STATIC_MIN_OVERFLOW: MinOverflow<StaticMaxCap<CAP>> = MinOverflow::<StaticMaxCap<CAP>>::new(OVER_CAP);
pub const STATIC_MAX_UNDERFLOW: MaxUnderflow<StaticMinCap<CAP>> = MaxUnderflow::<StaticMinCap<CAP>>::new(UNDER_CAP);
pub const STATIC_COMPAT_ERR_OVERFLOW: CompatError<StaticMinCap<CAP>, StaticMaxCap<CAP>> =
    CompatError::Overflow(STATIC_MIN_OVERFLOW);
pub const STATIC_COMPAT_ERR_UNDERFLOW: CompatError<StaticMinCap<CAP>, StaticMaxCap<CAP>> =
    CompatError::Underflow(STATIC_MAX_UNDERFLOW);

mod underflows {
    use super::*;

    mod dynamic {
        use super::*;

        check_eq!(new: MaxUnderflow::<MinCapVal>::new(UNDER_CAP, MIN_CAP_VAL) 
            => MAX_UNDERFLOW);
        panics!(panic_new: MaxUnderflow::<MinCapVal>::new(CAP, MIN_CAP_VAL) 
            => "max_size must be < min_cap");
        check_eq!(max_size: MAX_UNDERFLOW.max_size() => UNDER_CAP);
        check_eq!(min_cap: *MAX_UNDERFLOW.min_cap() => MIN_CAP_VAL);
    }

    mod static_cap {
        use super::*;

        check_eq!(new: MaxUnderflow::<StaticMinCap<CAP>>::new(UNDER_CAP) => STATIC_MAX_UNDERFLOW);
        panics!(panic_new: MaxUnderflow::<StaticMinCap<CAP>>::new(CAP) => "max_size must be < MIN");
        check_eq!(max_size: STATIC_MAX_UNDERFLOW.max_size() => UNDER_CAP);
        check_eq!(min_cap: *STATIC_MAX_UNDERFLOW.min_cap() => StaticMinCap::<CAP>::CAP);
    }
}

mod overflows {
    use super::*;

    mod dynamic {
        use super::*;

        check_eq!(new: MinOverflow::<MaxCapVal>::new(OVER_CAP, MAX_CAP_VAL) => MIN_OVERFLOW);
        panics!(panic_new: MinOverflow::<MaxCapVal>::new(CAP, MAX_CAP_VAL) 
            => "min_size must be > max_cap");
        check_eq!(min_size: MIN_OVERFLOW.min_size() => OVER_CAP);
        check_eq!(max_cap: *MIN_OVERFLOW.max_cap() => MAX_CAP_VAL);
    }

    mod static_cap {
        use super::*;

        check_eq!(new: MinOverflow::<StaticMaxCap<CAP>>::new(OVER_CAP) => STATIC_MIN_OVERFLOW);
        panics!(panic_new: MinOverflow::<StaticMaxCap<CAP>>::new(CAP) 
            => "min_size must be > MAX");
        check_eq!(min_size: STATIC_MIN_OVERFLOW.min_size() => OVER_CAP);
        check_eq!(max_cap: *STATIC_MIN_OVERFLOW.max_cap() => StaticMaxCap::<CAP>::CAP);
    }
}

mod errors {
    use super::*;

    mod dynamic {
        use super::*;

        check_eq!(from_overflow: CompatError::from(MIN_OVERFLOW) => COMPAT_ERR_OVERFLOW);
        check_eq!(from_underflow: CompatError::from(MAX_UNDERFLOW) => COMPAT_ERR_UNDERFLOW);
    }

    mod static_cap {
        use super::*;

        check_eq!(from_overflow: CompatError::from(STATIC_MIN_OVERFLOW) => STATIC_COMPAT_ERR_OVERFLOW);
        check_eq!(from_underflow: CompatError::from(STATIC_MAX_UNDERFLOW) => STATIC_COMPAT_ERR_UNDERFLOW);
    }
}
