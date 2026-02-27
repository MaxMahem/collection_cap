mod common;

use collection_cap::CapConstraint;
use collection_cap::cap::{ExactSize, MaxCapMarker, MinCapMarker, MinMaxCap};
use collection_cap::err::{CapError, CapOverflow, CapUnderflow};

use common::consts::*;
use common::{check_eq, panics};

mod min_cap_marker {
    use super::*;

    mod check_if_can_fit {
        use super::*;

        check_eq!(fits: MinCapMarker::<CAP>::check_if_can_fit(&FITS_ITER) => Ok(()));
        check_eq!(underflow: MinCapMarker::<CAP>::check_if_can_fit(&UNDER_ITER)
            => Err(CapUnderflow::new(CAP - 1)));

        panics!(bad_iter: MinCapMarker::<CAP>::check_if_can_fit(&INVALID_ITERATOR)
            => "Invalid size hint: InvalidSizeHint");
    }
}

mod max_cap_marker {
    use super::*;

    mod check_if_can_fit {
        use super::*;

        check_eq!(fits: MaxCapMarker::<CAP>::check_if_can_fit(&FITS_ITER) => Ok(()));
        check_eq!(overflow: MaxCapMarker::<CAP>::check_if_can_fit(&OVER_ITER)
            => Err(CapOverflow::new(CAP + 1)));

        panics!(bad_iter: MaxCapMarker::<CAP>::check_if_can_fit(&INVALID_ITERATOR)
            => "Invalid size hint: InvalidSizeHint");
    }
}

mod min_max_cap {
    use super::*;

    mod check_if_can_fit {
        use super::*;

        check_eq!(fits: MinMaxCap::<CAP, CAP>::check_if_can_fit(&FITS_ITER) => Ok(()));
        check_eq!(overflow: MinMaxCap::<CAP, CAP>::check_if_can_fit(&OVER_ITER)
            => Err(CapError::Overflow(CapOverflow::new(CAP + 1))));
        check_eq!(underflow: MinMaxCap::<CAP, CAP>::check_if_can_fit(&UNDER_ITER)
            => Err(CapError::Underflow(CapUnderflow::new(CAP - 1))));

        panics!(bad_iter: MinMaxCap::<CAP, CAP>::check_if_can_fit(&INVALID_ITERATOR)
            => "Invalid size hint: InvalidSizeHint");
    }
}

mod exact_size {
    use super::*;

    mod check_if_can_fit {
        use super::*;

        check_eq!(fits: ExactSize::<CAP>::check_if_can_fit(&FITS_ITER) => Ok(()));
        check_eq!(overflow: ExactSize::<CAP>::check_if_can_fit(&OVER_ITER)
            => Err(CapError::Overflow(CapOverflow::new(CAP + 1))));
        check_eq!(underflow: ExactSize::<CAP>::check_if_can_fit(&UNDER_ITER)
            => Err(CapError::Underflow(CapUnderflow::new(CAP - 1))));

        panics!(bad_iter: ExactSize::<CAP>::check_if_can_fit(&INVALID_ITERATOR)
            => "Invalid size hint: InvalidSizeHint");
    }
}
