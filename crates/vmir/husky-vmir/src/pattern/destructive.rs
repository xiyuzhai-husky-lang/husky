use idx_arena::ArenaIdx;

/// takes ownership of the match src, destruct it
#[derive(Debug, PartialEq, Eq)]
pub enum VmirDestructivePatternData {}

pub type VmirDestructivePatternIdx = ArenaIdx<VmirDestructivePatternData>;
