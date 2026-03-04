use derive_more::Display;

/// The upper bound of an iterator's size hint.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Display)]
pub enum UpperBound {
    /// A known, finite upper bound.
    #[display("{_0}")]
    Fixed(usize),
    /// No upper bound — the iterator may produce infinitely many elements.
    #[display("unbounded")]
    Unbounded,
}

impl From<Option<usize>> for UpperBound {
    fn from(opt: Option<usize>) -> Self {
        opt.map_or(Self::Unbounded, Self::Fixed)
    }
}

impl From<UpperBound> for Option<usize> {
    fn from(bound: UpperBound) -> Self {
        match bound {
            UpperBound::Fixed(n) => Some(n),
            UpperBound::Unbounded => None,
        }
    }
}
