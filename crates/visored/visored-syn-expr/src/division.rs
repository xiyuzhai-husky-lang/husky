use crate::stmt::{VdSynStmtIdx, VdSynStmtIdxRange};
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use latex_prelude::division::LxDivisionKind;
use smallvec::{smallvec, SmallVec};

#[derive(Debug, PartialEq, Eq)]
pub struct VdSynDivisionData {
    kind: LxDivisionKind,
    children: SmallVec<[VdSynDivisionChild; 4]>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSynDivisionChild {
    Division(VdSynDivisionIdx),
    Stmt(VdSynStmtIdx),
}

impl VdSynDivisionData {
    pub fn kind(&self) -> LxDivisionKind {
        self.kind
    }

    pub fn children(&self) -> &[VdSynDivisionChild] {
        &self.children
    }
}

pub type VdSynDivisionArena = Arena<VdSynDivisionData>;
pub type VdSynDivisionArenaRef<'a> = ArenaRef<'a, VdSynDivisionData>;
pub type VdSynDivisionMap<T> = ArenaMap<VdSynDivisionData, T>;
pub type VdSynDivisionIdx = ArenaIdx<VdSynDivisionData>;
pub type VdSynDivisionIdxRange = ArenaIdxRange<VdSynDivisionData>;
