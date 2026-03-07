use collection_cap::VariableCap;
use collection_cap::cap::MaxCapVal;

use crate::common::check_eq;
use crate::common::consts::*;

const MAX_CAP_VAL: MaxCapVal = MaxCapVal(CAP);

check_eq!(capacity: VariableCap::capacity(&[0i32; CAP][..]) => MAX_CAP_VAL);
