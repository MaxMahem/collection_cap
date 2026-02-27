use collection_cap::VariableCap;
use collection_cap::err::{Overflows, Underflows, VarCapError};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

mod max_cap_val {
    use super::*;
    use collection_cap::cap::MaxCapVal;

    mod check_compatibility {
        use super::*;

        check_eq!(compatible: MaxCapVal(CAP).check_compatability(&COMPAT_ITER) => Ok(()));
        check_eq!(overflow: MaxCapVal(CAP).check_compatability(&OVER_ITER)
            => Err(Overflows::new(CAP + 1, CAP)));

        panics!(bad_iter: MaxCapVal(CAP).check_compatability(&INVALID_ITERATOR)
            => "Invalid size hint");
    }
}

mod min_cap_val {
    use super::*;
    use collection_cap::cap::MinCapVal;

    mod check_compatibility {
        use super::*;

        check_eq!(compatible: MinCapVal(CAP).check_compatability(&COMPAT_ITER) => Ok(()));
        check_eq!(underflow: MinCapVal(CAP).check_compatability(&UNDER_ITER)
            => Err(Underflows::new(CAP - 1, CAP)));

        panics!(bad_iter: MinCapVal(CAP).check_compatability(&INVALID_ITERATOR)
            => "Invalid size hint");
    }
}

mod min_max_cap_val {
    use super::*;
    use collection_cap::cap::MinMaxCapVal;

    check_eq!(min_val: MinMaxCapVal::new(CAP, CAP).min() => CAP);
    check_eq!(max_val: MinMaxCapVal::new(CAP, CAP).max() => CAP);

    mod check_compatibility {
        use super::*;

        check_eq!(compatible: MinMaxCapVal::new(CAP, CAP).check_compatability(&COMPAT_ITER) => Ok(()));
        check_eq!(overflow: MinMaxCapVal::new(CAP, CAP).check_compatability(&OVER_ITER)
            => Err(COMPAT_ERROR_OVERFLOWS));
        check_eq!(underflow: MinMaxCapVal::new(CAP, CAP).check_compatability(&UNDER_ITER)
            => Err(COMPAT_ERROR_UNDERFLOWS));

        panics!(bad_iter: MinMaxCapVal::new(CAP, CAP).check_compatability(&INVALID_ITERATOR)
            => "Invalid size hint");

        panics!(invalid_range: MinMaxCapVal::new(CAP, CAP - 1)
            => "min cap must be <= max cap");
    }
}

mod exact_constraint {
    use super::*;
    use collection_cap::cap::ExactCapVal;

    mod check_compatibility {
        use super::*;

        check_eq!(compatible: ExactCapVal(CAP).check_compatability(&COMPAT_ITER) => Ok(()));
        check_eq!(overflow: ExactCapVal(CAP).check_compatability(&OVER_ITER)
            => Err(VarCapError::Overflows(Overflows::new(CAP + 1, CAP))));
        check_eq!(underflow: ExactCapVal(CAP).check_compatability(&UNDER_ITER)
            => Err(VarCapError::Underflows(Underflows::new(CAP - 1, CAP))));

        panics!(bad_iter: ExactCapVal(CAP).check_compatability(&INVALID_ITERATOR)
            => "Invalid size hint");
    }
}
