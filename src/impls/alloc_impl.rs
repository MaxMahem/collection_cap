use crate::cap::MaxCapVal;

/// An extension for dynamically allocated collections (like
/// [`Vec`](alloc::vec::Vec) or [`String`](alloc::string::String))
/// granting access to the available allocated capacity.
pub trait SpareCapacityExt {
    /// Returns the current spare capacity in the allocation.
    ///
    /// Spare capacity is calculated as `capacity - length`.
    ///
    /// # Example
    /// ```
    /// # use collection_cap::SpareCapacityExt;
    /// # use collection_cap::cap::MaxCapVal;
    /// let mut v = Vec::with_capacity(10);
    ///
    /// v.extend(0..4);
    ///
    /// let expected = v.capacity() - (0..4).len();
    /// assert_eq!(expected, 10 - 4);
    /// assert_eq!(v.spare_capacity(), MaxCapVal(expected));
    /// ```
    fn spare_capacity(&self) -> MaxCapVal;
}

impl<T> SpareCapacityExt for alloc::vec::Vec<T> {
    fn spare_capacity(&self) -> MaxCapVal {
        self.capacity().saturating_sub(self.len()).into()
    }
}

impl SpareCapacityExt for alloc::string::String {
    /// Returns the dynamically allocated spare capacity.
    ///
    /// Spare capacity is calculated as `capacity - length`.
    ///
    /// Note: The returned spare capacity is measured in **bytes**, not
    /// **characters**, matching the behavior of [`String::capacity`](alloc::string::String::capacity).
    ///
    /// # Example
    /// ```
    /// # use collection_cap::SpareCapacityExt;
    /// # use collection_cap::cap::MaxCapVal;
    /// let mut s = String::with_capacity(10);
    ///
    /// let crab = "🦀";
    /// assert_eq!(crab.len(), 4, "The crab emoji should be 4 bytes");
    /// s.push_str(crab);
    ///
    /// let expected = s.capacity() - crab.len();
    /// assert_eq!(expected, 10 - 4);
    /// assert_eq!(s.spare_capacity(), MaxCapVal(expected));
    /// ```
    fn spare_capacity(&self) -> MaxCapVal {
        self.capacity().saturating_sub(self.len()).into()
    }
}

impl<T> SpareCapacityExt for alloc::collections::VecDeque<T> {
    fn spare_capacity(&self) -> MaxCapVal {
        self.capacity().saturating_sub(self.len()).into()
    }
}
