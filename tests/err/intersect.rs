use collection_cap::cap::{MaxCapVal, MinCapVal};
use collection_cap::err::{IntersectError, MaxUnderflow, MinOverflow};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

const MAX_CAP_VAL: MaxCapVal = MaxCapVal(CAP);
const MIN_CAP_VAL: MinCapVal = MinCapVal(CAP);

pub const MIN_OVERFLOW: MinOverflow<MaxCapVal> = MinOverflow::<MaxCapVal>::new(OVER_CAP, MAX_CAP_VAL);
pub const MAX_UNDERFLOW: MaxUnderflow<MinCapVal> = MaxUnderflow::<MinCapVal>::new(UNDER_CAP, MIN_CAP_VAL);
pub const INTERSECT_ERR_OVERFLOW: IntersectError<MinCapVal, MaxCapVal> = IntersectError::Overflow(MIN_OVERFLOW);
pub const INTERSECT_ERR_UNDERFLOW: IntersectError<MinCapVal, MaxCapVal> = IntersectError::Underflow(MAX_UNDERFLOW);

pub const CONST_MIN_OVERFLOW: MinOverflow<ConstMaxCap<CAP>> = MinOverflow::<ConstMaxCap<CAP>>::new(OVER_CAP);
pub const CONST_MAX_UNDERFLOW: MaxUnderflow<ConstMinCap<CAP>> = MaxUnderflow::<ConstMinCap<CAP>>::new(UNDER_CAP);
pub const CONST_INTERSECT_ERR_OVERFLOW: IntersectError<ConstMinCap<CAP>, ConstMaxCap<CAP>> =
    IntersectError::Overflow(CONST_MIN_OVERFLOW);
pub const CONST_INTERSECT_ERR_UNDERFLOW: IntersectError<ConstMinCap<CAP>, ConstMaxCap<CAP>> =
    IntersectError::Underflow(CONST_MAX_UNDERFLOW);

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

    mod const_cap {
        use super::*;

        check_eq!(new: MaxUnderflow::<ConstMinCap<CAP>>::new(UNDER_CAP) => CONST_MAX_UNDERFLOW);
        panics!(panic_new: MaxUnderflow::<ConstMinCap<CAP>>::new(CAP) => "max_size must be < MIN");
        check_eq!(max_size: CONST_MAX_UNDERFLOW.max_size() => UNDER_CAP);
        check_eq!(min_cap: *CONST_MAX_UNDERFLOW.min_cap() => ConstMinCap::<CAP>::CAP);
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

    mod const_cap {
        use super::*;

        check_eq!(new: MinOverflow::<ConstMaxCap<CAP>>::new(OVER_CAP) => CONST_MIN_OVERFLOW);
        panics!(panic_new: MinOverflow::<ConstMaxCap<CAP>>::new(CAP) 
            => "min_size must be > MAX");
        check_eq!(min_size: CONST_MIN_OVERFLOW.min_size() => OVER_CAP);
        check_eq!(max_cap: *CONST_MIN_OVERFLOW.max_cap() => ConstMaxCap::<CAP>::CAP);
    }
}

mod errors {
    use super::*;

    mod dynamic {
        use super::*;

        check_eq!(from_overflow: IntersectError::from(MIN_OVERFLOW) => INTERSECT_ERR_OVERFLOW);
        check_eq!(from_underflow: IntersectError::from(MAX_UNDERFLOW) => INTERSECT_ERR_UNDERFLOW);
    }

    mod const_cap {
        use super::*;

        check_eq!(from_overflow: IntersectError::from(CONST_MIN_OVERFLOW) => CONST_INTERSECT_ERR_OVERFLOW);
        check_eq!(from_underflow: IntersectError::from(CONST_MAX_UNDERFLOW) => CONST_INTERSECT_ERR_UNDERFLOW);
    }
}
