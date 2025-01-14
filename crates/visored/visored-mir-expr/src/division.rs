use idx_arena::{Arena, ArenaIdx, ArenaIdxRange, ArenaRef};

pub enum VdMirDivisionData {}

pub struct VdMirDivisionEntry {
    data: VdMirDivisionData,
}

pub type VdMirDivisionIdx = ArenaIdx<VdMirDivisionEntry>;
pub type VdMirDivisionIdxRange = ArenaIdxRange<VdMirDivisionEntry>;
pub type VdMirDivisionArena = Arena<VdMirDivisionEntry>;
pub type VdMirDivisionArenaRef<'a> = ArenaRef<'a, VdMirDivisionEntry>;
