extern crate alloc;

use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::vec::Vec;

use collection_cap::SpareCapacityExt;
use collection_cap::cap::MaxCapVal;

use crate::common::iter::FoldMut;
use crate::common::check_eq;
use crate::common::consts::*;

macro_rules! test_spare_capacity_decreases {
    ($type:ty => $push:ident) => {
        #[test]
        fn spare_capacity_decreases() {
            let mut c = <$type>::with_capacity(CAP);

            let cap = c.spare_capacity().0;
            c.$push(Default::default());

            assert_eq!(c.spare_capacity(), MaxCapVal(cap - 1));
        }
    };
}

macro_rules! test_spare_capacity_full {
    ($type:ty => $push:ident) => {
        #[test]
        fn spare_capacity_full() {
            let c = (0..CAP)
                .map(|_| Default::default())
                .fold_mut(<$type>::with_capacity(CAP), <$type>::$push);

            assert_eq!(c.spare_capacity(), ZERO_MAX_CAP_VAL);
        }
    };
}

mod vec {
    use super::*;

    check_eq!(spare_capacity_empty: Vec::<i32>::with_capacity(CAP).spare_capacity() 
        => MaxCapVal(Vec::<i32>::with_capacity(CAP).capacity()));

    test_spare_capacity_decreases!(Vec<i32> => push);

    test_spare_capacity_full!(Vec<i32> => push);
}

mod string {
    use super::*;

    check_eq!(spare_capacity_empty: String::with_capacity(CAP).spare_capacity() 
        => MaxCapVal(String::with_capacity(CAP).capacity()));

    test_spare_capacity_decreases!(String => push);

    test_spare_capacity_full!(String => push);
}

mod vec_deque {
    use super::*;

    check_eq!(spare_capacity_empty: VecDeque::<i32>::with_capacity(CAP).spare_capacity() 
        => MaxCapVal(VecDeque::<i32>::with_capacity(CAP).capacity()));

    test_spare_capacity_decreases!(VecDeque<i32> => push_back);

    test_spare_capacity_full!(VecDeque<i32> => push_back);
}
