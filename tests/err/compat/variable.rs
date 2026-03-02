use collection_cap::err::{CapOverflow, CapUnderflow};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

mod overflows {
    use super::*;

    check_eq!(new: CapOverflow::new(OVER_CAP, MAX_CAP) => CAP_OVERFLOWS);
    panics!(panic_new: CapOverflow::new(CAP, MAX_CAP) => "min_size must be > max_cap");
    check_eq!(min_size: CAP_OVERFLOWS.min_size() => OVER_CAP);
    check_eq!(max_cap: CAP_OVERFLOWS.max_cap() => MAX_CAP);
}

mod underflows {
    use super::*;

    check_eq!(new: CapUnderflow::new(UNDER_CAP, MIN_CAP) => CAP_UNDERFLOWS);
    panics!(panic_new: CapUnderflow::new(CAP, MIN_CAP) => "max_size must be < min_cap");
    check_eq!(max_size: CAP_UNDERFLOWS.max_size() => UNDER_CAP);
    check_eq!(min_cap: CAP_UNDERFLOWS.min_cap() => MIN_CAP);
}
