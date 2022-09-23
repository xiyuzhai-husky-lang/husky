use crate::{Trackable, TrackableMakeChangeM, TrackableTakeChangeM};

#[derive(Default)]
pub struct TrackableAtom<V> {
    value: V,
    changed: bool,
}

pub enum TrackableAtomChange<V> {
    Some(V),
    None,
}

impl<V> Trackable for TrackableAtom<V>
where
    V: Clone,
{
    type Change = TrackableAtomChange<V>;

    fn take_change(&mut self) -> TrackableTakeChangeM<Self> {
        if self.changed {
            self.changed = false;
            TrackableTakeChangeM::Ok(TrackableAtomChange::Some(self.value.clone()))
        } else {
            TrackableTakeChangeM::Ok(TrackableAtomChange::None)
        }
    }
}

impl<V> std::ops::Deref for TrackableAtom<V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<V> TrackableAtom<V> {
    pub fn set(&mut self, new_value: V) -> TrackableMakeChangeM<Self, ()> {
        self.changed = true;
        self.value = new_value;
        TrackableMakeChangeM::Ok {
            cont: (),
            phantom_state: std::marker::PhantomData,
        }
    }
    pub fn update(&mut self, f: impl FnOnce(&mut V)) -> TrackableMakeChangeM<Self, ()> {
        self.changed = true;
        f(&mut self.value);
        TrackableMakeChangeM::Ok {
            cont: (),
            phantom_state: std::marker::PhantomData,
        }
    }
}
