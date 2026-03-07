mod exact_cap_val;
mod max_cap_val;
mod min_cap_val;
mod min_max_cap_val;

pub use exact_cap_val::*;
pub use max_cap_val::*;
pub use min_cap_val::*;
pub use min_max_cap_val::*;

macro_rules! impl_variable_cap_from_self {
    ($type:ty) => {
        impl crate::VariableCap for $type {
            type Cap = Self;

            fn capacity(&self) -> Self {
                *self
            }
        }
    };
}

pub(crate) use impl_variable_cap_from_self;
