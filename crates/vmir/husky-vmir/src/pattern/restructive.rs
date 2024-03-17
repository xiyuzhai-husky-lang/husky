use idx_arena::ArenaIdx;

/// takes (mutable) reference of the match src, keep it
#[derive(Debug, PartialEq, Eq)]
pub enum VmirRestructivePatternData {}

pub type VmirRestructivePatternIdx = ArenaIdx<VmirRestructivePatternData>;
