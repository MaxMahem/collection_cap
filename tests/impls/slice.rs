use collection_cap::VariableCap;

use crate::common::check_eq;
use crate::common::consts::*;

check_eq!(capacity: VariableCap::capacity(&[0i32; base::CAP][..]) => val::MAX_CAP_VAL);
