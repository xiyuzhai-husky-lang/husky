use std::marker::PhantomData;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnumFullMap<I: IsEnumIndex, T>(Vec<T>, PhantomData<I>);

impl<I: IsEnumIndex, T> std::ops::Index<I> for EnumFullMap<I, T> {
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        &self.0[index.index()]
    }
}

impl<I: IsEnumIndex, T> EnumFullMap<I, T> {
    pub fn new(f: impl Fn(I) -> T) -> Self {
        Self(
            (0..I::N).into_iter().map(|i| f(I::from_index(i))).collect(),
            PhantomData,
        )
    }

    pub fn indexed_iter<'a>(&'a self) -> impl Iterator<Item = (I, &'a T)> + 'a {
        self.0
            .iter()
            .enumerate()
            .map(|(i, t)| (I::from_index(i), t))
    }

    pub fn as_ref(&self) -> EnumFullMapRef<I, T> {
        EnumFullMapRef(self.0.as_ref(), PhantomData)
    }
}

pub struct EnumFullMapRef<'a, I: IsEnumIndex, T>(&'a [T], PhantomData<I>);

impl<'a, I: IsEnumIndex, T> std::ops::Index<I> for EnumFullMapRef<'a, I, T> {
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        &self.0[index.index()]
    }
}
