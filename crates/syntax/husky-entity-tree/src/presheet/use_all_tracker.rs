use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UseAllTracker {
    parent: EntityPath,
    // how many symbols have been checked
    progress: usize,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub(crate) struct UseAllTrackers(Vec<UseAllTracker>);

pub(crate) struct UseAllTrackerIdx(usize);

impl UseAllTrackers {
    pub(crate) fn indexed_iter(&self) -> impl Iterator<Item = (UseAllTrackerIdx, &UseAllTracker)> {
        self.0
            .iter()
            .enumerate()
            .map(|(i, tracker)| (UseAllTrackerIdx(i), tracker))
    }
}
