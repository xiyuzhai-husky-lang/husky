use std::marker::PhantomData;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnumFullVecMap<I: IsEnumIndex, T>(Vec<T>, PhantomData<I>);

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnumFullArrayMap<I: IsEnumIndex, T>([T; <I as IsEnumIndex>::N], PhantomData<I>)
where
    [(); <I as IsEnumIndex>::N]:;

impl<I: IsEnumIndex, T> std::ops::Index<I> for EnumFullVecMap<I, T> {
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        &self.0[index.index()]
    }
}

impl<I: IsEnumIndex, T> std::ops::IndexMut<I> for EnumFullVecMap<I, T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.0[index.index()]
    }
}

impl<I: IsEnumIndex, T> Default for EnumFullVecMap<I, T>
where
    T: Default,
{
    fn default() -> Self {
        Self(
            (0..I::N).into_iter().map(|_| Default::default()).collect(),
            Default::default(),
        )
    }
}

impl<I: IsEnumIndex, T> EnumFullVecMap<I, T> {
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
