use collection_cap::IterCapExt;
use collection_cap::cap::{ExactSize, MaxCapMarker, MinCapMarker, MinMaxCap};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

mod min_cap_marker {
    use super::*;

    check_eq!(compatible: COMPAT_ITER.ensure_compatible::<MinCapMarker<CAP>>() => Ok(()));
    check_eq!(underflow: UNDER_ITER.ensure_compatible::<MinCapMarker<CAP>>()
            => Err(CAP_UNDERFLOWS));

    panics!(bad_iter: INVALID_ITER.ensure_compatible::<MinCapMarker<CAP>>()
            => "Invalid size hint");
}

mod max_cap_marker {
    use super::*;

    check_eq!(compatible: COMPAT_ITER.ensure_compatible::<MaxCapMarker<CAP>>() => Ok(()));
    check_eq!(overflow: OVER_ITER.ensure_compatible::<MaxCapMarker<CAP>>()
            => Err(CAP_OVERFLOWS));

    panics!(bad_iter: INVALID_ITER.ensure_compatible::<MaxCapMarker<CAP>>()
            => "Invalid size hint");
}

mod min_max_cap {
    use super::*;

    check_eq!(compatible: COMPAT_ITER.ensure_compatible::<MinMaxCap<CAP, CAP>>() => Ok(()));
    check_eq!(overflow: OVER_ITER.ensure_compatible::<MinMaxCap<CAP, CAP>>()
            => Err(CAP_ERROR_OVERFLOW));
    check_eq!(underflow: UNDER_ITER.ensure_compatible::<MinMaxCap<CAP, CAP>>()
            => Err(CAP_ERROR_UNDERFLOW));

    panics!(bad_iter: INVALID_ITER.ensure_compatible::<MinMaxCap<CAP, CAP>>()
            => "Invalid size hint");
}

mod exact_size {
    use super::*;

    check_eq!(compatible: COMPAT_ITER.ensure_compatible::<ExactSize<CAP>>() => Ok(()));
    check_eq!(overflow: OVER_ITER.ensure_compatible::<ExactSize<CAP>>()
            => Err(CAP_ERROR_OVERFLOW));
    check_eq!(underflow: UNDER_ITER.ensure_compatible::<ExactSize<CAP>>()
            => Err(CAP_ERROR_UNDERFLOW));

    panics!(bad_iter: INVALID_ITER.ensure_compatible::<ExactSize<CAP>>()
            => "Invalid size hint");
}
