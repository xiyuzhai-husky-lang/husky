use serde::{Deserialize, Serialize};

use crate::Trackable;

#[derive(Default)]
pub struct TrackableAtom<V> {
    value: V,
    changed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TrackableAtomChange<V> {
    Some(V),
    None,
}

impl<V> Trackable for TrackableAtom<V>
where
    V: Clone,
{
    type Change = TrackableAtomChange<V>;

    fn take_change(&mut self) -> Self::Change {
        if self.changed {
            self.changed = false;
            TrackableAtomChange::Some(self.value.clone())
        } else {
            TrackableAtomChange::None
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
    pub fn set(&mut self, new_value: V) {
        self.changed = true;
        self.value = new_value
    }
    pub fn update(&mut self, f: impl FnOnce(&mut V)) {
        self.changed = true;
        f(&mut self.value)
    }

    pub fn clear_pop(&mut self) -> V
    where
        V: Default,
    {
        self.changed = false;
        std::mem::take(&mut self.value)
    }
}
