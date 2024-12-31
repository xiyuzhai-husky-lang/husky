use idx_arena::{
    map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef,
};
use once_place::OncePlace;
use visored_sem_expr::clause::VdSemClauseIdx;

use crate::stmt::VdMirStmtIdx;

#[derive(Debug, PartialEq, Eq)]
pub enum VdMirTacticData {
    Obvious,
}

#[derive(Debug, PartialEq, Eq)]
pub struct VdMirTacticEntry {
    data: VdMirTacticData,
}

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdMirTacticSource {
    Clause(VdSemClauseIdx),
}

pub type VdMirHintIdx = ArenaIdx<VdMirTacticEntry>;
pub type VdMirHintIdxRange = ArenaIdxRange<VdMirTacticEntry>;
pub type VdMirTacticArena = Arena<VdMirTacticEntry>;
pub type VdMirTacticMap<T> = ArenaMap<VdMirTacticEntry, T>;
pub type VdMirTacticOrderedMap<T> = ArenaOrderedMap<VdMirTacticEntry, T>;
pub type VdMirTacticArenaRef<'a> = ArenaRef<'a, VdMirTacticEntry>;
