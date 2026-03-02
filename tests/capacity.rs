#![allow(clippy::needless_borrow)]
mod common;

use collection_cap::IterCapExt;

use common::consts::*;
use common::{check_eq, panics};

mod variable_cap_ref {
    use super::*;

    check_eq!(compatible: COMPAT_ITER.ensure_compatible_with(&MAX_CAP) => Ok(()));
    check_eq!(overflow: OVER_ITER.ensure_compatible_with(&MAX_CAP) => Err(CAP_OVERFLOWS));
    check_eq!(underflow: UNDER_ITER.ensure_compatible_with(&MIN_CAP) => Err(CAP_UNDERFLOWS));

    panics!(bad_iter: INVALID_ITER.ensure_compatible_with(&MAX_CAP) => "Invalid size hint");
}
