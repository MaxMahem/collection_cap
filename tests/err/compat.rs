use collection_cap::err::{MaxUnderflow, MinOverflow};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

mod static_underflows {
    use super::*;

    const STATIC_MAX_UNDERFLOW: MaxUnderflow<MinMaxCap> 
        = MaxUnderflow::<MinMaxCap>::new_static(UNDER_CAP);

    check_eq!(new: MaxUnderflow::<MinMaxCap>::new_static(UNDER_CAP) 
        => STATIC_MAX_UNDERFLOW);
    panics!(panic_new: MaxUnderflow::<MinMaxCap>::new_static(CAP) 
        => "max_size must be < C::MIN_CAP");
    check_eq!(max_size: STATIC_MAX_UNDERFLOW.max_size() => UNDER_CAP);
}

mod static_overflows {
    use super::*;

    const STATIC_MIN_OVERFLOW: MinOverflow<MinMaxCap> 
        = MinOverflow::<MinMaxCap>::new_static(OVER_CAP);

    check_eq!(new: MinOverflow::<MinMaxCap>::new_static(OVER_CAP) 
        => STATIC_MIN_OVERFLOW);
    panics!(panic_new: MinOverflow::<MinMaxCap>::new_static(CAP) 
        => "min_size must be > C::MAX_CAP");
    check_eq!(min_size: STATIC_MIN_OVERFLOW.min_size() => OVER_CAP);
}

mod dynamic_overflows {
    use super::*;

    check_eq!(new: MinOverflow::new(OVER_CAP, MAX_CAP_VAL) => MIN_OVERFLOWS);
    panics!(panic_new: MinOverflow::new(CAP, MAX_CAP_VAL) => "min_size must be > max_cap");
    check_eq!(min_size: MIN_OVERFLOWS.min_size() => OVER_CAP);
    check_eq!(max_cap: MIN_OVERFLOWS.max_cap() => MAX_CAP_VAL);
}

mod dynamic_underflows {
    use super::*;

    check_eq!(new: MaxUnderflow::new(UNDER_CAP, MIN_CAP_VAL) => MAX_UNDERFLOWS);
    panics!(panic_new: MaxUnderflow::new(CAP, MIN_CAP_VAL) => "max_size must be < min_cap");
    check_eq!(max_size: MAX_UNDERFLOWS.max_size() => UNDER_CAP);
    check_eq!(min_cap: MAX_UNDERFLOWS.min_cap() => MIN_CAP_VAL);
}
