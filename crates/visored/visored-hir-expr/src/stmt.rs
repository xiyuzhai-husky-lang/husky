use crate::expr::VdHirExprIdx;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange, ArenaRef};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VdHirStmtData {
    // Add appropriate variants here, for example:
    Expression(VdHirExprIdx),
    Block(VdHirStmtIdxRange),
    // Add more variants as needed
}

pub type VdHirStmtArena = Arena<VdHirStmtData>;
pub type VdHirStmtArenaRef<'a> = ArenaRef<'a, VdHirStmtData>;
pub type VdHirStmtIdx = ArenaIdx<VdHirStmtData>;
pub type VdHirStmtIdxRange = ArenaIdxRange<VdHirStmtData>;
