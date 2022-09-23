use crate::*;
use vec_like::VecSet;

pub struct TrackableVec<E> {
    entries: Vec<E>,
    state: TrackableVecState,
}

#[derive(Default)]
pub struct TrackableVecState {
    old_len: usize,
    elems_modified: VecSet<usize>,
}

pub struct ProjVecChange;

impl<K> Trackable for TrackableVec<K> {
    type Change = ProjVecChange;

    fn take_change(&mut self) -> TrackableTakeChangeM<Self> {
        todo!()
    }
}

impl<E> TrackableVec<E> {
    fn synced(&self) -> bool {
        self.state.old_len == self.entries.len() && self.state.elems_modified.len() == 0
    }

    pub fn push(&mut self, elem: E) -> TrackableUpdateM<Self, ()> {
        self.entries.push(elem);
        TrackableUpdateM::Ok {
            cont: (),
            phantom_state: PhantomData,
        }
    }

    pub fn apply_set_elem(&mut self, index: usize, elem: E) -> TrackableApplyChangeM<Self, ()> {
        assert!(self.synced());
        todo!()
    }
}

impl<E> Default for TrackableVec<E> {
    fn default() -> Self {
        Self {
            entries: Default::default(),
            state: Default::default(),
        }
    }
}

impl<E> std::ops::Deref for TrackableVec<E> {
    type Target = [E];

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl<E> std::ops::Index<usize> for TrackableVec<E> {
    type Output = E;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl<E> std::ops::Index<std::ops::RangeFrom<usize>> for TrackableVec<E> {
    type Output = [E];

    fn index(&self, range: std::ops::RangeFrom<usize>) -> &Self::Output {
        &self.entries[range]
    }
}

impl<E> std::ops::IndexMut<usize> for TrackableVec<E> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.state.elems_modified.insert(index);
        &mut self.entries[index]
    }
}
