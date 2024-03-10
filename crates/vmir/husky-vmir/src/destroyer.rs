use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

pub struct VmirDestroyerData {}

pub type VmirDestroyerArena = Arena<VmirDestroyerData>;
pub type VmirDestroyerIdx = ArenaIdx<VmirDestroyerData>;
pub type VmirDestroyerIdxRange = ArenaIdxRange<VmirDestroyerData>;
