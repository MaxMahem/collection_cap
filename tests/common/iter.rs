#![allow(dead_code)]

pub(crate) trait FoldMut: Iterator {
    fn fold_mut<C>(self, init: C, mut f: impl FnMut(&mut C, Self::Item)) -> C
    where
        Self: Sized,
    {
        self.fold(init, |mut acc, x| {
            f(&mut acc, x);
            acc
        })
    }
}

impl<I: Iterator> FoldMut for I {}
