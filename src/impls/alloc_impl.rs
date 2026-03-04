use crate::cap::MaxCapVal;

/// An extension trait for dynamically allocated collections (like `Vec` or `String`)
/// to easily access their available capacity as a [`MaxCapVal`].
pub trait SpareCapacityExt {
    /// Returns the dynamically allocated spare capacity as a [`MaxCapVal`].
    ///
    /// Spare capacity is calculated as `capacity - length`.
    fn spare_capacity(&self) -> MaxCapVal;
}

impl<T> SpareCapacityExt for alloc::vec::Vec<T> {
    fn spare_capacity(&self) -> MaxCapVal {
        self.capacity().saturating_sub(self.len()).into()
    }
}

impl SpareCapacityExt for alloc::string::String {
    fn spare_capacity(&self) -> MaxCapVal {
        self.capacity().saturating_sub(self.len()).into()
    }
}

impl<T> SpareCapacityExt for alloc::collections::VecDeque<T> {
    fn spare_capacity(&self) -> MaxCapVal {
        self.capacity().saturating_sub(self.len()).into()
    }
}
