mod state;

pub use state::*;

use crate::*;
use serde::{Deserialize, Serialize};
use vec_like::VecSet;

pub trait TrackClone {
    type CloneOutput: Serialize;
    fn track_clone(&self) -> Self::CloneOutput;
}

pub struct TrackSimple<T>(T);

impl<T> TrackSimple<T> {
    pub fn unwrap(&self) -> T
    where
        T: Copy,
    {
        self.0
    }
}

impl<T> TrackClone for TrackSimple<T>
where
    T: Clone + Serialize,
{
    type CloneOutput = T;

    fn track_clone(&self) -> Self::CloneOutput {
        self.0.clone()
    }
}

impl<T> From<T> for TrackSimple<T> {
    fn from(t: T) -> Self {
        TrackSimple(t)
    }
}

pub struct TrackableVec<E>
where
    E: TrackClone,
{
    entries: Vec<E>,
    state: TrackableVecState,
}

pub type TrackableVecSimple<E> = TrackableVec<TrackSimple<E>>;

#[derive(Debug, Serialize, Deserialize)]
pub enum TrackableVecChange<ETrackeCloneOutput>
where
    ETrackeCloneOutput: Serialize,
{
    Incremental {
        modified_entries: Vec<(usize, ETrackeCloneOutput)>,
        new_entries: Vec<ETrackeCloneOutput>,
    },
}

impl<E> TrackableVec<E>
where
    E: TrackClone,
{
    pub fn push(&mut self, elem: E) {
        self.entries.push(elem)
    }

    pub fn set(&mut self, new_value: Vec<E>) {
        self.state = Default::default();
        self.entries = new_value
    }

    pub fn clear_pop(&mut self) -> Vec<E> {
        let entries = std::mem::take(&mut self.entries);
        self.state = Default::default();
        entries
    }

    pub fn set_elem(&mut self, index: usize, new_value: E) {
        self.entries[index] = new_value;
        self.state.modify_element(index)
    }

    pub fn update_elem(&mut self, index: usize, f: impl FnOnce(&mut E)) {
        f(&mut self.entries[index]);
        self.state.modify_element(index)
    }

    pub fn apply_set_elem(&mut self, _index: usize, _elem: E) {
        todo!()
    }

    pub fn apply_update_elem(&mut self, index: usize, f: impl FnOnce(&mut E)) {
        f(&mut self.entries[index]);
        self.state.modify_element(index)
    }

    pub fn state(&self) -> &TrackableVecState {
        &self.state
    }
}

impl<E> Default for TrackableVec<E>
where
    E: TrackClone,
{
    fn default() -> Self {
        Self {
            entries: Default::default(),
            state: Default::default(),
        }
    }
}

impl<E> std::ops::Deref for TrackableVec<E>
where
    E: TrackClone,
{
    type Target = [E];

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl<E> std::ops::Index<usize> for TrackableVec<E>
where
    E: TrackClone,
{
    type Output = E;

    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl<E> std::ops::Index<std::ops::RangeFrom<usize>> for TrackableVec<E>
where
    E: TrackClone,
{
    type Output = [E];

    fn index(&self, range: std::ops::RangeFrom<usize>) -> &Self::Output {
        &self.entries[range]
    }
}

// impl<E> std::ops::IndexMut<usize> for TrackableVec<E>
// where
//     E: TrackClone,
// {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         if index < self.state.old_len {
//             self.state.modify_element(index);
//         }
//         &mut self.entries[index]
//     }
// }
