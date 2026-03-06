#![allow(unused_imports)]
use crate::common::consts::*;
use collection_cap::err::UpperBound;

use crate::common::check_eq;


check_eq!(from_some: UpperBound::from(Some(base::CAP)) => UpperBound::Fixed(base::CAP));
check_eq!(from_none: UpperBound::from(None) => UpperBound::Unbounded);

check_eq!(to_some: Option::<usize>::from(UpperBound::Fixed(base::CAP)) => Some(base::CAP));
check_eq!(to_none: Option::<usize>::from(UpperBound::Unbounded) => None);

check_eq!(display_fixed: UpperBound::Fixed(base::CAP).to_string() => base::CAP.to_string());
check_eq!(display_unbounded: UpperBound::Unbounded.to_string() => "unbounded");
