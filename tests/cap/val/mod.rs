use core::ops::Bound::*;
use core::ops::RangeBounds;

use collection_cap::cap::{ExactCapVal, MaxCapVal, MinCapVal, MinMaxCapVal, UnboundedCap};
use collection_cap::cap::{StaticExactCap, StaticMaxCap, StaticMinCap, StaticMinMaxCap};
use collection_cap::err::{EmptyRange, FitError, InvalidRange, MaxOverflow, RangeError};
use collection_cap::{Capacity, VariableCap};

use crate::common::consts::*;
use crate::common::{check_eq, panics};

const MIN_CAP: MinCapVal = MinCapVal(base::CAP);
const MAX_CAP: MaxCapVal = MaxCapVal(base::CAP);
const MIN_MAX_CAP: MinMaxCapVal = MinMaxCapVal::new(base::CAP, base::CAP);
const EXACT_CAP: ExactCapVal = ExactCapVal(base::CAP);

mod exact_cap_val;
mod max_cap_val;
mod min_cap_val;
mod min_max_cap_val;
