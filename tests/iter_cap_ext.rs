mod common;

use crate::common::consts::iter::*;
use crate::common::consts::*;
use crate::common::{check_eq, panics};

use collection_cap::IterCapExt;
use collection_cap::cap::{ConstMaxCap, ConstMinCap, MaxCapVal, MinCapVal};
use collection_cap::err::{IntersectError, MaxUnderflow, MinOverflow};
use collection_cap::err::{OverlapError, OverlapErrorSpan, MaxOverflow, MinUnderflow};

type FixedCap = [i32; CAP];

mod ensure_intersects {
    use super::*;

    const MAX_UNDERFLOW: MaxUnderflow<ConstMinCap<CAP>> = MaxUnderflow::<ConstMinCap<CAP>>::new(UNDER_CAP);
    const MIN_OVERFLOW: MinOverflow<ConstMaxCap<CAP>> = MinOverflow::<ConstMaxCap<CAP>>::new(OVER_CAP);

    check_eq!(intersecting: INTERSECT_ITER.ensure_intersects::<FixedCap>() => Ok(()));
    check_eq!(underflow: UNDER_ITER.ensure_intersects::<FixedCap>()
        => Err(IntersectError::Underflow(MAX_UNDERFLOW)));
    check_eq!(overflow: OVER_ITER.ensure_intersects::<FixedCap>()
        => Err(IntersectError::Overflow(MIN_OVERFLOW)));

    panics!(bad_iter: INVALID_ITER.ensure_intersects::<FixedCap>() 
        => "Invalid size hint");

    #[test]
    fn dyn_iterator() {
        let iter: &dyn Iterator<Item = i32> = &INTERSECT_ITER;
        iter.ensure_intersects::<FixedCap>().expect("Should work for dyn Iterator");
    }
}

mod ensure_intersects_with {
    use super::*;

    const MIN_CAP: MinCapVal = MinCapVal(CAP);
    const MAX_CAP: MaxCapVal = MaxCapVal(CAP);
    const MAX_UNDERFLOW: MaxUnderflow<MinCapVal> = MaxUnderflow::<MinCapVal>::new(UNDER_CAP, MIN_CAP);
    const MIN_OVERFLOW: MinOverflow<MaxCapVal> = MinOverflow::<MaxCapVal>::new(OVER_CAP, MAX_CAP);

    check_eq!(intersecting: INTERSECT_ITER.ensure_intersects_with(CAP_RANGE) => Ok(()));
    check_eq!(overflow: OVER_ITER.ensure_intersects_with(CAP_RANGE) 
        => Err(IntersectError::Overflow(MIN_OVERFLOW)));
    check_eq!(underflow: UNDER_ITER.ensure_intersects_with(CAP_RANGE) 
        => Err(IntersectError::Underflow(MAX_UNDERFLOW)));

    panics!(bad_iter: INVALID_ITER.ensure_intersects_with(CAP_RANGE) 
        => "Invalid size hint");
}

mod ensure_overlaps {
    use super::*;

    const MIN_UNDERFLOW: MinUnderflow<ConstMinCap<CAP>> = MinUnderflow::<ConstMinCap<CAP>>::new(UNDER_CAP);
    const MAX_OVERFLOW: MaxOverflow<ConstMaxCap<CAP>> = MaxOverflow::<ConstMaxCap<CAP>>::fixed(OVER_CAP);
    const OVERLAP_ERROR_SPAN: OverlapErrorSpan<ConstMinCap<CAP>, ConstMaxCap<CAP>> =
        OverlapErrorSpan::new(MAX_OVERFLOW, MIN_UNDERFLOW);

    check_eq!(overlap: INTERSECT_ITER.ensure_overlaps::<FixedCap>() => Ok(()));
    check_eq!(underflow: UNDER_ITER.ensure_overlaps::<FixedCap>()
        => Err(OverlapError::Underflow(MIN_UNDERFLOW)));
    check_eq!(overflow: OVER_ITER.ensure_overlaps::<FixedCap>()
        => Err(OverlapError::Overflow(MAX_OVERFLOW)));
    check_eq!(both: BOTH_ITER.ensure_overlaps::<FixedCap>()
        => Err(OverlapError::Both(OVERLAP_ERROR_SPAN)));

    panics!(bad_iter: INVALID_ITER.ensure_overlaps::<FixedCap>() 
        => "Invalid size hint");

    #[test]
    fn dyn_iterator() {
        let iter: &dyn Iterator<Item = i32> = &INTERSECT_ITER;
        iter.ensure_overlaps::<FixedCap>().expect("Should work for dyn Iterator");
    }
}

mod ensure_overlaps_into {
    use super::*;

    const MIN_CAP: MinCapVal = MinCapVal(CAP);
    const MAX_CAP: MaxCapVal = MaxCapVal(CAP);
    const MIN_UNDERFLOW: MinUnderflow<MinCapVal> = MinUnderflow::<MinCapVal>::new(UNDER_CAP, MIN_CAP);
    const MAX_OVERFLOW: MaxOverflow<MaxCapVal> = MaxOverflow::<MaxCapVal>::fixed(OVER_CAP, MAX_CAP);
    const OVERLAP_ERROR_SPAN: OverlapErrorSpan<MinCapVal, MaxCapVal> = OverlapErrorSpan::new(MAX_OVERFLOW, MIN_UNDERFLOW);

    check_eq!(overlap: INTERSECT_ITER.ensure_overlaps_into(CAP_RANGE) => Ok(()));
    check_eq!(overflow: OVER_ITER.ensure_overlaps_into(CAP_RANGE) 
        => Err(OverlapError::Overflow(MAX_OVERFLOW)));
    check_eq!(underflow: UNDER_ITER.ensure_overlaps_into(CAP_RANGE) 
        => Err(OverlapError::Underflow(MIN_UNDERFLOW)));
    check_eq!(both: BOTH_ITER.ensure_overlaps_into(CAP_RANGE) 
        => Err(OverlapError::Both(OVERLAP_ERROR_SPAN)));

    panics!(bad_iter: INVALID_ITER.ensure_overlaps_into(CAP_RANGE) 
        => "Invalid size hint");
}
