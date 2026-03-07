use crate::common::consts::*;
use collection_cap::err::UpperBound;

use crate::common::check_eq;

check_eq!(from_some: UpperBound::from(Some(CAP)) => UpperBound::Fixed(CAP));
check_eq!(from_none: UpperBound::from(None) => UpperBound::Unbounded);

check_eq!(to_some: Option::<usize>::from(UpperBound::Fixed(CAP)) => Some(CAP));
check_eq!(to_none: Option::<usize>::from(UpperBound::Unbounded) => None);

check_eq!(display_fixed: UpperBound::Fixed(CAP).to_string() => CAP.to_string());
check_eq!(display_unbounded: UpperBound::Unbounded.to_string() => "unbounded");
