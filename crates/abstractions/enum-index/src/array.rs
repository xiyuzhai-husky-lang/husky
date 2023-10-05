use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnumArray<I: IsEnumIndex, T>([T; I::N])
where
    [(); I::N]:;

impl<I: IsEnumIndex, T> std::ops::Index<I> for EnumArray<I, T>
where
    [(); I::N]:,
{
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        &self.0[index.index()]
    }
}

impl<I: IsEnumIndex, T> EnumArray<I, T>
where
    [(); I::N]:,
{
    pub fn indexed_iter<'a>(&'a self) -> impl Iterator<Item = (I, &'a T)> + 'a {
        self.0
            .iter()
            .enumerate()
            .map(|(i, t)| (I::from_index(i), t))
    }
}
