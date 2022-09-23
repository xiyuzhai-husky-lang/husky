use crate::*;
use vec_like::VecSet;

pub trait TrackClone {
    type Output;
    fn track(&self) -> Self::Output;
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
    T: Clone,
{
    type Output = T;

    fn track(&self) -> Self::Output {
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

#[derive(Default)]
pub struct TrackableVecState {
    old_len: usize,
    elems_modified: VecSet<usize>,
}

pub enum TrackableVecChange<E>
where
    E: TrackClone,
{
    None,
    Append { new_entries: Vec<E> },
}

impl<E> Trackable for TrackableVec<E>
where
    E: TrackClone,
{
    type Change = TrackableVecChange<E>;

    fn take_change(&mut self) -> TrackableTakeChangeM<Self> {
        if self.state.elems_modified.len() > 0 {
            println!("{:?} {}", self.state.elems_modified, self.state.old_len);
            todo!()
        }
        if self.state.old_len == self.entries.len() {
            return TrackableTakeChangeM::Ok(TrackableVecChange::None);
        }
        let new_entries: Vec<_> = self.entries[self.state.old_len..]
            .iter()
            .map(|v| v.clone())
            .collect();
        self.state.old_len = self.entries.len();
        todo!()
    }
}

impl<E> TrackableVec<E>
where
    E: TrackClone,
{
    fn synced(&self) -> bool {
        self.state.old_len == self.entries.len() && self.state.elems_modified.len() == 0
    }

    pub fn push(&mut self, elem: E) -> TrackableMakeChangeM<Self, ()> {
        self.entries.push(elem);
        TrackableMakeChangeM::Ok {
            cont: (),
            phantom_state: PhantomData,
        }
    }

    pub fn set(&mut self, new_value: Vec<E>) -> TrackableMakeChangeM<Self, ()> {
        self.state = Default::default();
        self.entries = new_value;
        TrackableMakeChangeM::Ok {
            cont: (),
            phantom_state: PhantomData,
        }
    }

    pub fn apply_set_elem(&mut self, index: usize, elem: E) -> TrackableApplyChangeM<Self, ()> {
        assert!(self.synced());
        todo!()
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

impl<E> std::ops::IndexMut<usize> for TrackableVec<E>
where
    E: TrackClone,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index < self.state.old_len {
            self.state.elems_modified.insert(index);
        }
        &mut self.entries[index]
    }
}
