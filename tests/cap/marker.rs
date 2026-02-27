use collection_cap::StaticCap;
use collection_cap::cap::{ExactSize, MaxCapMarker, MinCapMarker, MinMaxCap};
use collection_cap::err::{CapError, CapOverflow, CapUnderflow};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

mod min_cap_marker {
    use super::*;

    mod check_compatibility {
        use super::*;

        check_eq!(compatible: MinCapMarker::<CAP>::check_compatability(&COMPAT_ITER) => Ok(()));
        check_eq!(underflow: MinCapMarker::<CAP>::check_compatability(&UNDER_ITER)
            => Err(CapUnderflow::new(CAP - 1)));

        panics!(bad_iter: MinCapMarker::<CAP>::check_compatability(&INVALID_ITERATOR)
            => "Invalid size hint");
    }
}

mod max_cap_marker {
    use super::*;

    mod check_compatibility {
        use super::*;

        check_eq!(compatible: MaxCapMarker::<CAP>::check_compatability(&COMPAT_ITER) => Ok(()));
        check_eq!(overflow: MaxCapMarker::<CAP>::check_compatability(&OVER_ITER)
            => Err(CapOverflow::new(CAP + 1)));

        panics!(bad_iter: MaxCapMarker::<CAP>::check_compatability(&INVALID_ITERATOR)
            => "Invalid size hint");
    }
}

mod min_max_cap {
    use super::*;

    mod check_compatibility {
        use super::*;

        check_eq!(compatible: MinMaxCap::<CAP, CAP>::check_compatability(&COMPAT_ITER) => Ok(()));
        check_eq!(overflow: MinMaxCap::<CAP, CAP>::check_compatability(&OVER_ITER)
            => Err(CapError::Overflow(CapOverflow::new(CAP + 1))));
        check_eq!(underflow: MinMaxCap::<CAP, CAP>::check_compatability(&UNDER_ITER)
            => Err(CapError::Underflow(CapUnderflow::new(CAP - 1))));

        panics!(bad_iter: MinMaxCap::<CAP, CAP>::check_compatability(&INVALID_ITERATOR)
            => "Invalid size hint");
    }
}

mod exact_size {
    use super::*;

    mod check_compatibility {
        use super::*;

        check_eq!(compatible: ExactSize::<CAP>::check_compatability(&COMPAT_ITER) => Ok(()));
        check_eq!(overflow: ExactSize::<CAP>::check_compatability(&OVER_ITER)
            => Err(CapError::Overflow(CapOverflow::new(CAP + 1))));
        check_eq!(underflow: ExactSize::<CAP>::check_compatability(&UNDER_ITER)
            => Err(CapError::Underflow(CapUnderflow::new(CAP - 1))));

        panics!(bad_iter: ExactSize::<CAP>::check_compatability(&INVALID_ITERATOR)
            => "Invalid size hint");
    }
}
