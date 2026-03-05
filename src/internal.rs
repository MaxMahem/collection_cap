//! Internal implementation details.

/// A sealed trait used to restrict trait implementations to the crate.
pub trait Sealed {}

pub const EMPTY_RANGE_MSG: &str = "Range must not be empty";
pub const INVALID_RANGE_MSG: &str = "Invalid range (start > end)";

macro_rules! assert_then {
    ($cond:expr => $then:expr $(, $arg:tt)*) => {{
        assert!($cond $(, $arg)*);
        $then
    }};
}

pub(crate) use assert_then;
