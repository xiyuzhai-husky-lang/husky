use super::*;

#[derive(Debug, Default)]
pub struct TrackableVecState {
    old_len: usize,
    elems_modified: VecSet<usize>,
}

impl TrackableVecState {
    pub(super) fn modify_element(&mut self, index: usize) {
        if index < self.old_len {
            self.elems_modified.insert(index)
        }
    }
}

impl<E> Trackable for TrackableVec<E>
where
    E: TrackClone,
{
    type Change = TrackableVecChange<E::CloneOutput>;

    fn take_change(&mut self) -> Self::Change {
        let change = if self.state.old_len > self.entries.len() {
            todo!("non incremental")
        } else {
            let modified_entries = self
                .state
                .elems_modified
                .iter()
                .map(|i| (*i, self.entries[*i].track_clone()))
                .collect();
            let new_entries = self.entries[self.state.old_len..]
                .iter()
                .map(|v| v.track_clone())
                .collect::<Vec<_>>();
            self.state.old_len = self.entries.len();
            TrackableVecChange::Incremental {
                modified_entries,
                new_entries,
            }
        };
        self.state = TrackableVecState {
            old_len: self.entries.len(),
            elems_modified: Default::default(),
        };
        change
    }
}
