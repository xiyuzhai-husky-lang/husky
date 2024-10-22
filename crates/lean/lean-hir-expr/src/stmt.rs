mod tactics;

use idx_arena::{Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use lean_coword::ident::LnIdent;
use smallvec::SmallVec;
use tactics::LnHirTactics;

#[derive(Debug, PartialEq, Eq)]
pub enum LnHirStmtData {
    Tactics(LnHirTactics),
}

pub type LnHirStmtArena = Arena<LnHirStmtData>;
pub type LnHirStmtArenaRef<'a> = ArenaRef<'a, LnHirStmtData>;
pub type LnHirStmtIdx = ArenaIdx<LnHirStmtData>;
pub type LnHirStmtIdxRange = ArenaIdxRange<LnHirStmtData>;
