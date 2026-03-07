mod common;

use collection_cap::IterCapExt;

use collection_cap::cap::{MaxCapVal, MinCapVal};
use collection_cap::err::{MaxUnderflow, MinOverflow};
use common::consts::*;
use common::{check_eq, panics};

const MAX_CAP_VAL: MaxCapVal = MaxCapVal(CAP);
const MIN_CAP_VAL: MinCapVal = MinCapVal(CAP);

const MIN_OVERFLOW: MinOverflow<MaxCapVal> = MinOverflow::<MaxCapVal>::new(OVER_CAP, MAX_CAP_VAL);
const MAX_UNDERFLOW: MaxUnderflow<MinCapVal> = MaxUnderflow::<MinCapVal>::new(UNDER_CAP, MIN_CAP_VAL);

mod variable_cap_ref {
    use super::*;

    check_eq!(compatible: iter::COMPAT_ITER.ensure_compatible_with(&MAX_CAP_VAL) => Ok(()));
    check_eq!(overflow: iter::OVER_ITER.ensure_compatible_with(&MAX_CAP_VAL) => Err(MIN_OVERFLOW));
    check_eq!(underflow: iter::UNDER_ITER.ensure_compatible_with(&MIN_CAP_VAL) => Err(MAX_UNDERFLOW));

    panics!(bad_iter: iter::INVALID_ITER.ensure_compatible_with(&MAX_CAP_VAL) => "Invalid size hint");
}

mod variable_cap_mut_ref {
    use super::*;

    check_eq!(compatible: {
        let mut cap = MAX_CAP_VAL;
        iter::COMPAT_ITER.ensure_compatible_with(&mut cap)
    } => Ok(()));
    check_eq!(overflow: {
        let mut cap = MAX_CAP_VAL;
        iter::OVER_ITER.ensure_compatible_with(&mut cap)
    } => Err(MIN_OVERFLOW));
    check_eq!(underflow: {
        let mut cap = MIN_CAP_VAL;
        iter::UNDER_ITER.ensure_compatible_with(&mut cap)
    } => Err(MAX_UNDERFLOW));

    panics!(bad_iter: {
        let mut cap = MAX_CAP_VAL;
        iter::INVALID_ITER.ensure_compatible_with(&mut cap)
    } => "Invalid size hint");
}
