mod common;

use collection_cap::ValConstraint;
use collection_cap::err::{FitError, Overflows, Underflows};

use common::consts::*;
use common::{check_eq, panics};

mod max_constraint {
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

mod min_constraint {
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

mod min_max_constraint {
    use super::*;
    use collection_cap::cap::MinMaxCapVal;

    mod check_if_can_fit {
        use super::*;

        check_eq!(fits: MinMaxCapVal::new(CAP, CAP).check_if_can_fit(&FITS_ITER) => Ok(()));
        check_eq!(overflow: MinMaxCapVal::new(CAP, CAP).check_if_can_fit(&OVER_ITER)
            => Err(FitError::Overflows(Overflows::new(CAP + 1, CAP))));
        check_eq!(underflow: MinMaxCapVal::new(CAP, CAP).check_if_can_fit(&UNDER_ITER)
            => Err(FitError::Underflows(Underflows::new(CAP - 1, CAP))));

        panics!(bad_iter: MinMaxCapVal::new(CAP, CAP).check_if_can_fit(&INVALID_ITERATOR)
            => "Invalid size hint");

        panics!(invalid_range: MinMaxCapVal::new(CAP, CAP - 1)
            => "min capacity must be less than or equal to max capacity");
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

mod range_to {
    use super::*;

    mod check_if_can_fit {
        use super::*;

        // ..CAP means max is CAP - 1
        check_eq!(fits: (..CAP).check_if_can_fit(&UNDER_ITER) => Ok(()));
        check_eq!(overflow: (..CAP).check_if_can_fit(&FITS_ITER)
            => Err(Overflows::new(CAP, CAP - 1)));

        panics!(empty_range: (..0).check_if_can_fit(&FITS_ITER)
            => "capacity constraint range must not be empty");
    }
}

mod range_to_inclusive {
    use super::*;

    mod check_if_can_fit {
        use super::*;

        // ..=CAP means max is CAP
        check_eq!(fits: (..=CAP).check_if_can_fit(&FITS_ITER) => Ok(()));
        check_eq!(overflow: (..=CAP).check_if_can_fit(&OVER_ITER)
            => Err(Overflows::new(CAP + 1, CAP)));
    }
}

mod range_from {
    use super::*;

    mod check_if_can_fit {
        use super::*;

        // CAP.. means min is CAP
        check_eq!(fits: (CAP..).check_if_can_fit(&FITS_ITER) => Ok(()));
        check_eq!(underflow: (CAP..).check_if_can_fit(&UNDER_ITER)
            => Err(Underflows::new(CAP - 1, CAP)));
    }
}

mod range {
    use super::*;

    mod check_if_can_fit {
        use super::*;

        // 0..CAP means min=0, max=CAP-1
        check_eq!(fits: (0..CAP).check_if_can_fit(&UNDER_ITER) => Ok(()));
        check_eq!(overflow: (0..CAP).check_if_can_fit(&FITS_ITER)
            => Err(FitError::Overflows(Overflows::new(CAP, CAP - 1))));

        panics!(empty_range: (core::ops::Range { start: CAP, end: CAP }).check_if_can_fit(&FITS_ITER)
            => "capacity constraint range must not be empty");
    }
}

mod range_inclusive {
    use super::*;

    mod check_if_can_fit {
        use super::*;

        // 0..=CAP means min=0, max=CAP
        check_eq!(fits: (0..=CAP).check_if_can_fit(&FITS_ITER) => Ok(()));
        check_eq!(overflow: (0..=CAP).check_if_can_fit(&OVER_ITER)
            => Err(FitError::Overflows(Overflows::new(CAP + 1, CAP))));

        panics!(empty_range: (core::ops::RangeInclusive::new(CAP, CAP - 1)).check_if_can_fit(&FITS_ITER)
            => "capacity constraint range must not be empty");
    }
}
mod range_full {
    use super::*;

    mod check_if_can_fit {
        use super::*;

        // .. means any size fits
        check_eq!(fits: (..).check_if_can_fit(&FITS_ITER) => Ok(()));
        check_eq!(fits_over: (..).check_if_can_fit(&OVER_ITER) => Ok(()));
        check_eq!(fits_under: (..).check_if_can_fit(&UNDER_ITER) => Ok(()));
    }
}
