#![allow(clippy::needless_borrow)]
mod common;

use collection_cap::IterCapExt;

use common::consts::*;
use common::{check_eq, panics};

mod variable_cap_ref {
    use super::*;

    check_eq!(compatible: COMPAT_ITER.ensure_compatible_with(&MAX_CAP_VAL) => Ok(()));
    check_eq!(overflow: OVER_ITER.ensure_compatible_with(&MAX_CAP_VAL) => Err(MIN_OVERFLOWS));
    check_eq!(underflow: UNDER_ITER.ensure_compatible_with(&MIN_CAP_VAL) => Err(MAX_UNDERFLOWS));

    panics!(bad_iter: INVALID_ITER.ensure_compatible_with(&MAX_CAP_VAL) => "Invalid size hint");
}

mod variable_cap_mut_ref {
    use super::*;

    check_eq!(compatible: COMPAT_ITER.ensure_compatible_with(&mut MAX_CAP_VAL.clone()) => Ok(()));
    check_eq!(overflow: OVER_ITER.ensure_compatible_with(&mut MAX_CAP_VAL.clone()) => Err(MIN_OVERFLOWS));
    check_eq!(underflow: UNDER_ITER.ensure_compatible_with(&mut MIN_CAP_VAL.clone()) => Err(MAX_UNDERFLOWS));

    panics!(bad_iter: INVALID_ITER.ensure_compatible_with(&mut MAX_CAP_VAL.clone()) => "Invalid size hint");
}
