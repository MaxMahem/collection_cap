use collection_cap::cap::{MaxCapVal, StaticMaxCap};
use collection_cap::{StaticCap, VariableCap};

use crate::common::check_eq;
use crate::common::consts::*;

use arrayvec::ArrayVec;

type TestArrayVec = ArrayVec<i32, { base::CAP }>;

check_eq!(variable_capacity_empty: VariableCap::capacity(&TestArrayVec::new()) => val::MAX_CAP_VAL);
check_eq!(static_capacity: TestArrayVec::CAP => StaticMaxCap::<{ base::CAP }>);

#[test]
fn variable_capacity_decreases() {
    let mut coll = TestArrayVec::new();
    coll.push(0);
    assert_eq!(VariableCap::capacity(&coll), MaxCapVal(base::CAP - 1));
}

check_eq!(variable_capacity_full: VariableCap::capacity(&ArrayVec::from([0; base::CAP])) 
    => MaxCapVal::ZERO);
