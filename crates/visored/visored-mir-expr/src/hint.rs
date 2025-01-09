use idx_arena::{
    map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef,
};
use once_place::OncePlace;
use visored_sem_expr::clause::VdSemClauseIdx;

use crate::stmt::VdMirStmtIdx;

#[derive(Debug, PartialEq, Eq)]
pub enum VdMirHintData {}

#[derive(Debug, PartialEq, Eq)]
pub struct VdMirHintEntry {
    data: VdMirHintData,
}

#[enum_class::from_variants]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdMirHintSource {
    Clause(VdSemClauseIdx),
}

pub type VdMirHintIdx = ArenaIdx<VdMirHintEntry>;
pub type VdMirHintIdxRange = ArenaIdxRange<VdMirHintEntry>;
pub type VdMirHintArena = Arena<VdMirHintEntry>;
pub type VdMirHintMap<T> = ArenaMap<VdMirHintEntry, T>;
pub type VdMirHintOrderedMap<T> = ArenaOrderedMap<VdMirHintEntry, T>;
pub type VdMirHintArenaRef<'a> = ArenaRef<'a, VdMirHintEntry>;
