use collection_cap::cap::{MaxCapVal, MinCapVal};
use collection_cap::err::{MaxUnderflow, MinOverflow};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

mod static_underflows {
    use super::*;

    check_eq!(new: MaxUnderflow::<MinMaxCap>::new(UNDER_CAP) 
        => MaxUnderflow::<MinMaxCap>::new(UNDER_CAP));
    check_eq!(max_size: MaxUnderflow::<MinMaxCap>::new(UNDER_CAP).max_size() => UNDER_CAP);
}

mod static_overflows {
    use super::*;

    check_eq!(new: MinOverflow::<MinMaxCap>::new(OVER_CAP) 
        => MinOverflow::<MinMaxCap>::new(OVER_CAP));
    check_eq!(min_size: MinOverflow::<MinMaxCap>::new(OVER_CAP).min_size() => OVER_CAP);
}

mod dynamic_overflows {
    use super::*;

    check_eq!(new: MinOverflow::<MaxCapVal>::new(OVER_CAP, MAX_CAP_VAL) => MIN_OVERFLOWS);
    panics!(panic_new: MinOverflow::<MaxCapVal>::new(CAP, MAX_CAP_VAL) => "min_size must be > max_cap");
    check_eq!(min_size: MIN_OVERFLOWS.min_size() => OVER_CAP);
    check_eq!(max_cap: MIN_OVERFLOWS.max_cap() => &MAX_CAP_VAL);
}

mod dynamic_underflows {
    use super::*;

    check_eq!(new: MaxUnderflow::<MinCapVal>::new(UNDER_CAP, MIN_CAP_VAL) => MAX_UNDERFLOWS);
    panics!(panic_new: MaxUnderflow::<MinCapVal>::new(CAP, MIN_CAP_VAL) => "max_size must be < min_cap");
    check_eq!(max_size: MAX_UNDERFLOWS.max_size() => UNDER_CAP);
    check_eq!(min_cap: MAX_UNDERFLOWS.min_cap() => &MIN_CAP_VAL);
}
