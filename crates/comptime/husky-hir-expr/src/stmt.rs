use crate::*;
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange};

#[derive(Debug, PartialEq, Eq)]
pub enum HirStmt {}

pub type HirStmtArena = Arena<HirStmt>;
pub type HirStmtIdx = ArenaIdx<HirStmt>;
pub type HirStmtIdxRange = ArenaIdxRange<HirStmt>;
pub type HirStmtMap<V> = ArenaMap<HirStmt, V>;
