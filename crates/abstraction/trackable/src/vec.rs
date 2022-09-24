use crate::*;
use vec_like::VecSet;

pub trait TrackClone {
    type CloneOutput;
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
    T: Clone,
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

#[derive(Debug, Default)]
pub struct TrackableVecState {
    old_len: usize,
    elems_modified: VecSet<usize>,
}

pub enum TrackableVecChange<E>
where
    E: TrackClone,
{
    None,
    Append { new_entries: Vec<E::CloneOutput> },
}

impl<E> TrackableVecChange<E>
where
    E: TrackClone,
{
    pub fn opt_new_entries(self) -> Option<Vec<E::CloneOutput>> {
        match self {
            TrackableVecChange::None => None,
            TrackableVecChange::Append { new_entries } => Some(new_entries),
        }
    }
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
        let new_entries: Vec<E::CloneOutput> = self.entries[self.state.old_len..]
            .iter()
            .map(|v| v.track_clone())
            .collect();
        self.state.old_len = self.entries.len();
        TrackableTakeChangeM::Ok(TrackableVecChange::Append { new_entries })
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
    pub fn set_elem(&mut self, index: usize, new_value: E) -> TrackableMakeChangeM<Self, ()> {
        if index < self.state.old_len {
            todo!()
        }
        self.entries[index] = new_value;
        TrackableMakeChangeM::Ok {
            cont: (),
            phantom_state: PhantomData,
        }
    }

    pub fn apply_set_elem(&mut self, index: usize, elem: E) -> TrackableApplyChangeM<Self, ()> {
        assert!(self.synced());
        todo!()
    }

    pub fn apply_update_elem(
        &mut self,
        index: usize,
        f: impl FnOnce(&mut E),
    ) -> TrackableApplyChangeM<Self, ()> {
        assert!(self.synced());
        f(&mut self.entries[index]);
        TrackableApplyChangeM::Ok {
            this: PhantomData,
            cont: (),
        }
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
//             self.state.elems_modified.insert(index);
//         }
//         &mut self.entries[index]
//     }
// }
