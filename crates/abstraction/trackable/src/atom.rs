use crate::{Trackable, TrackableUpdateM};

#[derive(Default)]
pub struct TrackableAtom<V> {
    value: V,
    updated: bool,
}

pub struct TrackableAtomChange;

impl<V> Trackable for TrackableAtom<V> {
    type Change = TrackableAtomChange;

    fn take_change(&mut self) -> crate::TrackableTakeChangeM<Self> {
        todo!()
    }
}

impl<V> std::ops::Deref for TrackableAtom<V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<V> TrackableAtom<V> {
    pub fn set(&mut self, new_value: V) -> TrackableUpdateM<Self, ()> {
        self.updated = false;
        self.value = new_value;
        TrackableUpdateM::Ok {
            cont: (),
            phantom_state: std::marker::PhantomData,
        }
    }
    pub fn update(&mut self, f: impl FnOnce(&mut V)) -> TrackableUpdateM<Self, ()> {
        self.updated = false;
        f(&mut self.value);
        TrackableUpdateM::Ok {
            cont: (),
            phantom_state: std::marker::PhantomData,
        }
    }
}
