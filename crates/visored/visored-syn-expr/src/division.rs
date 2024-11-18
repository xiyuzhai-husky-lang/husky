use crate::{
    builder::ToVdSyn,
    stmt::{VdSynStmtIdx, VdSynStmtIdxRange},
};
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use latex_ast::ast::{rose::LxRoseAstIdxRange, LxAstIdxRange};
use latex_prelude::division::LxDivisionKind;
use latex_token::idx::LxTokenIdxRange;
use smallvec::{smallvec, SmallVec};

#[derive(Debug, PartialEq, Eq)]
pub enum VdSynDivisionData {
    Stmts {
        stmts: VdSynStmtIdxRange,
    },
    Divisions {
        kind: LxDivisionKind,
        divisions: VdSynDivisionIdxRange,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VdSynDivisionChild {
    Division(VdSynDivisionIdx),
    Stmt(VdSynStmtIdx),
}

impl VdSynDivisionData {
    pub fn kind(&self) -> LxDivisionKind {
        match *self {
            VdSynDivisionData::Stmts { .. } => LxDivisionKind::Stmts,
            VdSynDivisionData::Divisions { kind, .. } => kind,
        }
    }

    pub fn children(&self) -> Vec<VdSynDivisionChild> {
        match *self {
            VdSynDivisionData::Stmts { stmts } => stmts
                .into_iter()
                .map(|stmt| VdSynDivisionChild::Stmt(stmt))
                .collect(),
            VdSynDivisionData::Divisions { divisions, .. } => divisions
                .into_iter()
                .map(|division| VdSynDivisionChild::Division(division))
                .collect(),
        }
    }
}

pub type VdSynDivisionArena = Arena<VdSynDivisionData>;
pub type VdSynDivisionArenaRef<'a> = ArenaRef<'a, VdSynDivisionData>;
pub type VdSynDivisionMap<T> = ArenaMap<VdSynDivisionData, T>;
pub type VdSynDivisionIdx = ArenaIdx<VdSynDivisionData>;
pub type VdSynDivisionIdxRange = ArenaIdxRange<VdSynDivisionData>;

impl ToVdSyn<VdSynDivisionIdxRange> for (LxTokenIdxRange, LxRoseAstIdxRange) {
    fn to_vd_syn(self, builder: &mut crate::builder::VdSynExprBuilder) -> VdSynDivisionIdxRange {
        todo!()
    }
}
