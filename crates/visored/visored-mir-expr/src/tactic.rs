pub mod elaboration;

use elaboration::VdMirTacticElaborationTracker;
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use once_place::OncePlace;

pub enum VdMirTacticData {
    Obvious,
}

pub struct VdMirTacticEntry {
    data: VdMirTacticData,
    elaboration_tracker: OncePlace<VdMirTacticElaborationTracker>,
}

pub type VdMirTacticIdx = ArenaIdx<VdMirTacticEntry>;
pub type VdMirTacticIdxRange = ArenaIdxRange<VdMirTacticEntry>;
pub type VdMirTacticArena = Arena<VdMirTacticEntry>;
pub type VdMirTacticMap<T> = ArenaMap<VdMirTacticEntry, T>;
pub type VdMirTacticArenaRef<'a> = ArenaRef<'a, VdMirTacticEntry>;

impl VdMirTacticEntry {
    pub fn new(data: VdMirTacticData) -> Self {
        Self {
            data,
            elaboration_tracker: OncePlace::default(),
        }
    }

    #[track_caller]
    pub(crate) fn set_elaboration_tracker(
        &mut self,
        elaboration_tracker: VdMirTacticElaborationTracker,
    ) {
        self.elaboration_tracker.set(elaboration_tracker);
    }
}

impl VdMirTacticEntry {
    pub fn data(&self) -> &VdMirTacticData {
        &self.data
    }

    #[track_caller]
    pub fn elaboration_tracker(&self) -> &VdMirTacticElaborationTracker {
        &*self.elaboration_tracker
    }
}
