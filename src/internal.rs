//! Internal implementation details.

/// A sealed trait used to restrict trait implementations to the crate.
pub trait Sealed {}

macro_rules! assert_then {
    ($cond:expr => $then:expr $(, $arg:tt)*) => {{
        assert!($cond $(, $arg)*);
        $then
    }};
}

macro_rules! Ok {
    () => {{ Ok(()) }};
}

pub(crate) use Ok;
pub(crate) use assert_then;
