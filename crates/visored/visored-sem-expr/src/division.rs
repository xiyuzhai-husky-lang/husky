pub mod helpers;
#[cfg(test)]
mod tests;

use crate::stmt::VdSemStmtIdxRange;
use crate::*;
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use latex_token::idx::LxRoseTokenIdx;
use visored_prelude::division::VdDivisionLevel;
use visored_syn_expr::division::{VdSynDivisionData, VdSynDivisionIdx, VdSynDivisionIdxRange};

#[derive(Debug, PartialEq, Eq)]
pub enum VdSemDivisionData {
    Stmts {
        stmts: VdSemStmtIdxRange,
    },
    Divisions {
        command_token_idx: LxRoseTokenIdx,
        level: VdDivisionLevel,
        lcurl_token_idx: LxRoseTokenIdx,
        title: VdSemStmtIdxRange,
        rcurl_token_idx: LxRoseTokenIdx,
        subdivisions: VdSemDivisionIdxRange,
    },
}

pub type VdSemDivisionArena = Arena<VdSemDivisionData>;
pub type VdSemDivisionArenaRef<'a> = ArenaRef<'a, VdSemDivisionData>;
pub type VdSemDivisionMap<T> = ArenaMap<VdSemDivisionData, T>;
pub type VdSemDivisionIdx = ArenaIdx<VdSemDivisionData>;
pub type VdSemDivisionIdxRange = ArenaIdxRange<VdSemDivisionData>;

impl ToVdSem<VdSemDivisionIdxRange> for VdSynDivisionIdxRange {
    fn to_vd_sem(self, builder: &mut VdSemExprBuilder) -> VdSemDivisionIdxRange {
        let mut divisions: Vec<VdSemDivisionData> = vec![];
        for division in self {
            divisions.push(builder.build_division(division));
        }
        builder.alloc_divisions(divisions)
    }
}

impl<'a> VdSemExprBuilder<'a> {
    pub(crate) fn build_division(&mut self, division: VdSynDivisionIdx) -> VdSemDivisionData {
        match self.syn_division_arena()[division] {
            VdSynDivisionData::Stmts { stmts } => VdSemDivisionData::Stmts {
                stmts: stmts.to_vd_sem(self),
            },
            VdSynDivisionData::Divisions {
                command_token_idx,
                level,
                lcurl_token_idx,
                title,
                rcurl_token_idx,
                subdivisions,
            } => VdSemDivisionData::Divisions {
                command_token_idx,
                level,
                lcurl_token_idx,
                title: title.to_vd_sem(self),
                rcurl_token_idx,
                subdivisions: subdivisions.to_vd_sem(self),
            },
        }
    }
}
