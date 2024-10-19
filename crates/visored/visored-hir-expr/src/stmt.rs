use crate::expr::VisoredHirExprIdx;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VisoredHirStmtData {
    // Add appropriate variants here, for example:
    Expression(VisoredHirExprIdx),
    Block(VisoredHirStmtIdxRange),
    // Add more variants as needed
}

pub type VisoredHirStmtArena = Arena<VisoredHirStmtData>;
pub type VisoredHirStmtIdx = ArenaIdx<VisoredHirStmtData>;
pub type VisoredHirStmtIdxRange = ArenaIdxRange<VisoredHirStmtData>;
