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

        check_eq!(new: MaxUnderflow::<MinCapVal>::new(base::UNDER_CAP, val::MIN_CAP_VAL) => err_val_compat::MAX_UNDERFLOWS);
        panics!(panic_new: MaxUnderflow::<MinCapVal>::new(base::CAP, val::MIN_CAP_VAL) => "max_size must be < min_cap");
        check_eq!(max_size: err_val_compat::MAX_UNDERFLOWS.max_size() => base::UNDER_CAP);
        check_eq!(min_cap: *err_val_compat::MAX_UNDERFLOWS.min_cap() => val::MIN_CAP_VAL);
    }

    mod static_cap {
        use super::*;

        check_eq!(new: MaxUnderflow::<stat::MinCap>::new(base::UNDER_CAP) 
            => MaxUnderflow::<stat::MinCap>::new(base::UNDER_CAP));
        panics!(panic_new: MaxUnderflow::<stat::MinCap>::new(base::CAP) => "max_size must be < MIN");
        check_eq!(max_size: MaxUnderflow::<stat::MinCap>::new(base::UNDER_CAP).max_size() => base::UNDER_CAP);
        check_eq!(min_cap: *MaxUnderflow::<stat::MinCap>::new(base::UNDER_CAP).min_cap() => stat::MinCap::CAP);
    }
}

mod overflows {
    use super::*;

    mod dynamic {
        use super::*;

        check_eq!(new: MinOverflow::<MaxCapVal>::new(base::OVER_CAP, val::MAX_CAP_VAL) => err_val_compat::MIN_OVERFLOWS);
        panics!(panic_new: MinOverflow::<MaxCapVal>::new(base::CAP, val::MAX_CAP_VAL) => "min_size must be > max_cap");
        check_eq!(min_size: err_val_compat::MIN_OVERFLOWS.min_size() => base::OVER_CAP);
        check_eq!(max_cap: *err_val_compat::MIN_OVERFLOWS.max_cap() => val::MAX_CAP_VAL);
    }

    mod static_cap {
        use super::*;

        check_eq!(new: MinOverflow::<stat::MaxCap>::new(base::OVER_CAP) 
            => MinOverflow::<stat::MaxCap>::new(base::OVER_CAP));
        panics!(panic_new: MinOverflow::<stat::MaxCap>::new(base::CAP) => "min_size must be > MAX");
        check_eq!(min_size: MinOverflow::<stat::MaxCap>::new(base::OVER_CAP).min_size() => base::OVER_CAP);
        check_eq!(max_cap: *MinOverflow::<stat::MaxCap>::new(base::OVER_CAP).max_cap() => stat::MaxCap::CAP);
    }
}

mod errors {
    use super::*;

    mod dynamic {
        use super::*;

        check_eq!(from_overflow: CompatError::from(err_val_compat::MIN_OVERFLOWS) => err_val_compat::OVERFLOW);
        check_eq!(from_underflow: CompatError::from(err_val_compat::MAX_UNDERFLOWS) => err_val_compat::UNDERFLOW);
    }

    mod static_cap {
        use super::*;

        check_eq!(from_overflow: MinOverflow::<stat::MaxCap>::new(base::OVER_CAP)
            .pipe(CompatError::<stat::MinCap, stat::MaxCap>::from)
            => MinOverflow::<stat::MaxCap>::new(base::OVER_CAP).pipe(CompatError::Overflow));
        check_eq!(from_underflow: MaxUnderflow::<stat::MinCap>::new(base::UNDER_CAP)
            .pipe(CompatError::<stat::MinCap, stat::MaxCap>::from)
            => MaxUnderflow::<stat::MinCap>::new(base::UNDER_CAP).pipe(CompatError::Underflow));
    }
}
