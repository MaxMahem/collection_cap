use collection_cap::cap::{MaxCapVal, StaticMaxCap};
use collection_cap::{StaticCap, VariableCap};

use crate::common::check_eq;
use crate::common::consts::*;

use arrayvec::ArrayVec;

type TestArrayVec = ArrayVec<i32, CAP>;

check_eq!(variable_capacity_empty: VariableCap::capacity(&TestArrayVec::new()) => MAX_CAP_VAL);
check_eq!(static_capacity: TestArrayVec::CAP => StaticMaxCap::<CAP>);

#[test]
fn variable_capacity_decreases() {
    let mut coll = TestArrayVec::new();
    coll.push(0);
    assert_eq!(VariableCap::capacity(&coll), MaxCapVal(CAP - 1));
}

check_eq!(variable_capacity_full: VariableCap::capacity(&ArrayVec::from([0; CAP])) 
    => ZERO_MAX_CAP_VAL);
