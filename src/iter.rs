use size_hinter::SizeHint;
use tap::TryConv;

pub trait IterExt {
    /// The error message to use when the size hint is invalid.
    const INVALID_SIZE_HINT_MSG: &'static str = "Invalid size hint";

    /// Returns a valid size hint of the iterator.
    ///
    /// # Panics
    ///
    /// Panics if the size hint is invalid.
    fn valid_size_hint(&self) -> (usize, Option<usize>);
}

impl<I: Iterator + ?Sized> IterExt for I {
    #[track_caller]
    fn valid_size_hint(&self) -> (usize, Option<usize>) {
        self.size_hint().try_conv::<SizeHint>().map_or_else(|_| panic!("{}", Self::INVALID_SIZE_HINT_MSG), Into::into)
    }
}
