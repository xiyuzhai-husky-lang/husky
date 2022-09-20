use crate::*;
use projectable::{Projectable, Projector};
use vec_like::VecSet;

pub struct ProjVec<E> {
    entries: Vec<E>,
    state: ProjVecState,
}

#[derive(Default)]
pub struct ProjVecState {
    old_len: usize,
    elems_modified: VecSet<usize>,
}

impl<E> ProjVec<E> {
    pub fn push(&mut self, elem: E) -> ProjUpdatingM<Self, ()> {
        todo!()
    }
}

impl<E> Default for ProjVec<E> {
    fn default() -> Self {
        Self {
            entries: Default::default(),
            state: Default::default(),
        }
    }
}

impl<E> std::ops::Deref for ProjVec<E> {
    type Target = [E];

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl<E> std::ops::Index<usize> for ProjVec<E> {
    type Output = E;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl<E> std::ops::Index<std::ops::RangeFrom<usize>> for ProjVec<E> {
    type Output = [E];

    fn index(&self, range: std::ops::RangeFrom<usize>) -> &Self::Output {
        &self.entries[range]
    }
}

impl<E> std::ops::IndexMut<usize> for ProjVec<E> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.state.elems_modified.insert(index);
        &mut self.entries[index]
    }
}
