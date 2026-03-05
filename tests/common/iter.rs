#![allow(dead_code)]

pub(crate) trait IterExt: Iterator {
    fn fold_mut<C>(self, init: C, mut f: impl FnMut(&mut C, Self::Item)) -> C
    where
        Self: Sized,
    {
        self.fold(init, |mut acc, x| {
            f(&mut acc, x);
            acc
        })
    }

    fn map_to_default<TOut: Default>(self) -> impl Iterator<Item = TOut>
    where
        Self: Sized,
    {
        self.map(|_| TOut::default())
    }
}

impl<I: Iterator> IterExt for I {}
