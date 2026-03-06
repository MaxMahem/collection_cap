use core::ops::Bound::*;
use core::ops::RangeBounds;

use collection_cap::Capacity;
use collection_cap::cap::{StaticExactCap, StaticMaxCap, StaticMinCap, StaticMinMaxCap, UnboundedCap};
use collection_cap::err::{CompatError, FitError, FitErrorSpan, MaxOverflow, MaxUnderflow, MinOverflow, MinUnderflow};
use core::ops::{RangeFrom, RangeInclusive, RangeToInclusive};

use crate::common::{check_eq, panics};

mod exact_cap_stat;
mod max_cap_stat;
mod min_cap_stat;
mod min_max_cap_stat;
