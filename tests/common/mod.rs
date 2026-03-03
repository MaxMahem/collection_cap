#![allow(unused_macros)]
#![allow(unused_imports)]

pub mod consts;

macro_rules! check_eq {
    ($name:ident: $res:expr => $expected:expr) => {
        #[test]
        fn $name() {
            let result = $res;
            let expected = $expected;

            assert_eq!(result, expected);
        }
    };
}

macro_rules! panics {
    ($name:ident: $res:expr => $expected:literal) => {
        #[test]
        #[should_panic(expected = $expected)]
        fn $name() {
            _ = $res;
        }
    };
}

pub(crate) use check_eq;
pub(crate) use panics;
