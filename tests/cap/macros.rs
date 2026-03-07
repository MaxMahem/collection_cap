macro_rules! contains_size {
    ($cap:expr => { cap: $cap_res:expr, under: $under_res:expr, over: $over_res:expr }) => {
        mod contains_size {
            use super::*;
            use collection_cap::Capacity;
            use crate::common::consts::{CAP, UNDER_CAP, OVER_CAP};

            crate::common::check_eq!(contains_cap: $cap.contains_size(CAP) => $cap_res);
            crate::common::check_eq!(contains_under: $cap.contains_size(UNDER_CAP) => $under_res);
            crate::common::check_eq!(contains_over: $cap.contains_size(OVER_CAP) => $over_res);
        }
    };
}

macro_rules! caps {
    ($cap:expr => { min: $min:expr, max: $max:expr }) => {
        mod caps {
            use super::*;
            use collection_cap::Capacity;

            crate::common::check_eq!(min_cap: $cap.min_cap() => $min);
            crate::common::check_eq!(max_cap: $cap.max_cap() => $max);
        }
    };
}

macro_rules! check_compat {
    ($cap:expr => { overflow: $over_res:expr, underflow: $under_res:expr }) => {
        mod check_compat {
            use super::*;
            use collection_cap::Capacity;
            use crate::common::consts::iter::*;

            crate::common::check_eq!(compatible: $cap.check_compatibility(&COMPAT_ITER) => Ok(()));
            crate::common::check_eq!(overflow: $cap.check_compatibility(&OVER_ITER) => $over_res);
            crate::common::check_eq!(underflow: $cap.check_compatibility(&UNDER_ITER) => $under_res);

            crate::common::panics!(bad_iter: $cap.check_compatibility(&INVALID_ITER) => "Invalid size hint");
        }
    };
}

macro_rules! check_fit {
    ($cap:expr => { underflow: $under_res:expr, overflow: $over_res:expr, unbounded: $unbounded_res:expr, both: $both_res:expr }) => {
        mod check_fit {
            use super::*;
            use collection_cap::Capacity;
            use crate::common::consts::iter::*;

            crate::common::check_eq!(compatible: $cap.check_fit(&COMPAT_ITER) => Ok(()));
            crate::common::check_eq!(underflow: $cap.check_fit(&UNDER_ITER) => $under_res);
            crate::common::check_eq!(overflow: $cap.check_fit(&OVER_ITER) => $over_res);
            crate::common::check_eq!(overflow_unbounded: $cap.check_fit(&OVER_ITER_UNBOUNDED)
                => $unbounded_res);
            crate::common::check_eq!(both: $cap.check_fit(&BOTH_ITER) => $both_res);

            crate::common::panics!(bad_iter: $cap.check_fit(&INVALID_ITER) => "Invalid size hint");
        }
    };
}

macro_rules! range_bounds {
    ($cap:expr => { start: $start:expr, end: $end:expr }) => {
        mod range_bounds {
            use super::*;
            use core::ops::RangeBounds;

            crate::common::check_eq!(start_bound: $cap.start_bound() => $start);
            crate::common::check_eq!(end_bound: $cap.end_bound() => $end);
        }
    };
}

pub(crate) use caps;
pub(crate) use check_compat;
pub(crate) use check_fit;
pub(crate) use contains_size;
pub(crate) use range_bounds;
