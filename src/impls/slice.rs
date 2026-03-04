use crate::VariableCap;
use crate::cap::MaxCapVal;

impl<T> VariableCap for [T] {
    type Cap = MaxCapVal;

    fn capacity(&self) -> MaxCapVal {
        MaxCapVal(self.len())
    }
}
