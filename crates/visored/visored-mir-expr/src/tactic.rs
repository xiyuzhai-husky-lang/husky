pub mod elaboration;

use elaboration::VdMirTacticElaboration;
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use once_place::OncePlace;

pub enum VdMirTacticData {
    Obvious,
}

pub struct VdMirTacticEntry {
    data: VdMirTacticData,
    elaboration: OncePlace<VdMirTacticElaboration>,
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
            elaboration: OncePlace::default(),
        }
    }

    #[track_caller]
    pub(crate) fn set_elaboration(&mut self, elaboration: VdMirTacticElaboration) {
        self.elaboration.set(elaboration);
    }
}

impl VdMirTacticEntry {
    pub fn data(&self) -> &VdMirTacticData {
        &self.data
    }

    #[track_caller]
    pub fn elaboration(&self) -> &VdMirTacticElaboration {
        &*self.elaboration
    }
}
