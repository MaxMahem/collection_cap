//! Internal implementation details.

/// A sealed trait used to restrict trait implementations to the crate.
pub trait Sealed {}

macro_rules! impl_sealed {
    ($type:ty) => {
        impl crate::internal::Sealed for $type {}
    };
}

pub(crate) use impl_sealed;

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
