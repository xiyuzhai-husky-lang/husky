use crate::*;

pub struct TrackableSet<K> {
    entries: Vec<K>,
}

pub struct TrackableSetChange;

impl<K> Trackable for TrackableSet<K> {
    type Change = TrackableSetChange;

    fn take_change(&mut self) -> TrackableTakeChangeM<Self> {
        todo!()
    }
}
