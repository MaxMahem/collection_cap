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

macro_rules! check_intersects {
    ($cap:expr => { overflow: $over_res:expr, underflow: $under_res:expr }) => {
        mod check_intersects {
            use super::*;
            use collection_cap::Capacity;
            use crate::common::consts::iter::*;

            crate::common::check_eq!(intersecting: $cap.check_intersects(&INTERSECT_ITER) => Ok(()));
            crate::common::check_eq!(overflow: $cap.check_intersects(&OVER_ITER) => $over_res);
            crate::common::check_eq!(underflow: $cap.check_intersects(&UNDER_ITER) => $under_res);

            crate::common::panics!(bad_iter: $cap.check_intersects(&INVALID_ITER) => "Invalid size hint");
        }
    };
}

macro_rules! check_overlaps {
    ($cap:expr => { underflow: $under_res:expr, overflow: $over_res:expr, unbounded: $unbounded_res:expr, both: $both_res:expr }) => {
        mod check_overlaps {
            use super::*;
            use collection_cap::Capacity;
            use crate::common::consts::iter::*;

            crate::common::check_eq!(intersecting: $cap.check_overlaps(&INTERSECT_ITER) => Ok(()));
            crate::common::check_eq!(underflow: $cap.check_overlaps(&UNDER_ITER) => $under_res);
            crate::common::check_eq!(overflow: $cap.check_overlaps(&OVER_ITER) => $over_res);
            crate::common::check_eq!(overflow_unbounded: $cap.check_overlaps(&OVER_ITER_UNBOUNDED)
                => $unbounded_res);
            crate::common::check_eq!(both: $cap.check_overlaps(&BOTH_ITER) => $both_res);

            crate::common::panics!(bad_iter: $cap.check_overlaps(&INVALID_ITER) => "Invalid size hint");
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
pub(crate) use check_intersects;
pub(crate) use check_overlaps;
pub(crate) use contains_size;
pub(crate) use range_bounds;
