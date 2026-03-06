#![allow(clippy::needless_borrow)]
mod common;

use collection_cap::IterCapExt;

use common::consts::*;
use common::{check_eq, panics};

mod variable_cap_ref {
    use super::*;

    check_eq!(compatible: iter::COMPAT_ITER.ensure_compatible_with(&val::MAX_CAP_VAL) => Ok(()));
    check_eq!(overflow: iter::OVER_ITER.ensure_compatible_with(&val::MAX_CAP_VAL) => Err(err_val_compat::MIN_OVERFLOWS));
    check_eq!(underflow: iter::UNDER_ITER.ensure_compatible_with(&val::MIN_CAP_VAL) => Err(err_val_compat::MAX_UNDERFLOWS));

    panics!(bad_iter: iter::INVALID_ITER.ensure_compatible_with(&val::MAX_CAP_VAL) => "Invalid size hint");
}

mod variable_cap_mut_ref {
    use super::*;

    check_eq!(compatible: iter::COMPAT_ITER.ensure_compatible_with(&mut val::MAX_CAP_VAL.clone()) => Ok(()));
    check_eq!(overflow: iter::OVER_ITER.ensure_compatible_with(&mut val::MAX_CAP_VAL.clone()) => Err(err_val_compat::MIN_OVERFLOWS));
    check_eq!(underflow: iter::UNDER_ITER.ensure_compatible_with(&mut val::MIN_CAP_VAL.clone()) => Err(err_val_compat::MAX_UNDERFLOWS));

    panics!(bad_iter: iter::INVALID_ITER.ensure_compatible_with(&mut val::MAX_CAP_VAL.clone()) => "Invalid size hint");
}
