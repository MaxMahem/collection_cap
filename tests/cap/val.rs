use collection_cap::ValConstraint;
use collection_cap::err::{FitError, Overflows, Underflows};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

mod max_cap_val {
    use super::*;
    use collection_cap::cap::MaxCapVal;

    mod check_if_can_fit {
        use super::*;

        check_eq!(fits: MaxCapVal(CAP).check_if_can_fit(&FITS_ITER) => Ok(()));
        check_eq!(overflow: MaxCapVal(CAP).check_if_can_fit(&OVER_ITER)
            => Err(Overflows::new(CAP + 1, CAP)));

        panics!(bad_iter: MaxCapVal(CAP).check_if_can_fit(&INVALID_ITERATOR)
            => "Invalid size hint");
    }
}

mod min_cap_val {
    use super::*;
    use collection_cap::cap::MinCapVal;

    mod check_if_can_fit {
        use super::*;

        check_eq!(fits: MinCapVal(CAP).check_if_can_fit(&FITS_ITER) => Ok(()));
        check_eq!(underflow: MinCapVal(CAP).check_if_can_fit(&UNDER_ITER)
            => Err(Underflows::new(CAP - 1, CAP)));

        panics!(bad_iter: MinCapVal(CAP).check_if_can_fit(&INVALID_ITERATOR)
            => "Invalid size hint");
    }
}

mod min_max_cap_val {
    use super::*;
    use collection_cap::cap::MinMaxCapVal;

    check_eq!(min_val: MinMaxCapVal::new(CAP, CAP).min() => CAP);
    check_eq!(max_val: MinMaxCapVal::new(CAP, CAP).max() => CAP);

    mod check_if_can_fit {
        use super::*;

        check_eq!(fits: MinMaxCapVal::new(CAP, CAP).check_if_can_fit(&FITS_ITER) => Ok(()));
        check_eq!(overflow: MinMaxCapVal::new(CAP, CAP).check_if_can_fit(&OVER_ITER)
            => Err(FIT_ERROR_OVERFLOWS));
        check_eq!(underflow: MinMaxCapVal::new(CAP, CAP).check_if_can_fit(&UNDER_ITER)
            => Err(FIT_ERROR_UNDERFLOWS));

        panics!(bad_iter: MinMaxCapVal::new(CAP, CAP).check_if_can_fit(&INVALID_ITERATOR)
            => "Invalid size hint");

        panics!(invalid_range: MinMaxCapVal::new(CAP, CAP - 1)
            => "min cap must be <= max cap");
    }
}

mod exact_constraint {
    use super::*;
    use collection_cap::cap::ExactCapVal;

    mod check_if_can_fit {
        use super::*;

        check_eq!(fits: ExactCapVal(CAP).check_if_can_fit(&FITS_ITER) => Ok(()));
        check_eq!(overflow: ExactCapVal(CAP).check_if_can_fit(&OVER_ITER)
            => Err(FitError::Overflows(Overflows::new(CAP + 1, CAP))));
        check_eq!(underflow: ExactCapVal(CAP).check_if_can_fit(&UNDER_ITER)
            => Err(FitError::Underflows(Underflows::new(CAP - 1, CAP))));

        panics!(bad_iter: ExactCapVal(CAP).check_if_can_fit(&INVALID_ITERATOR)
            => "Invalid size hint");
    }
}
