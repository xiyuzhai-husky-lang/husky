#[cfg(test)]
mod tests;

use crate::sentence::VdSemSentenceIdxRange;
use crate::*;
use husky_coword::Coword;
use idx_arena::{
    map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef,
};
use latex_environment::signature::LxEnvironmentSignature;
use latex_token::idx::LxRoseTokenIdx;
use sentence::VdSemSentenceIdx;
use visored_syn_expr::stmt::{VdSynStmtData, VdSynStmtIdx};

pub enum VdSemStmtData {
    Paragraph(VdSemSentenceIdxRange),
    Environment {
        begin_command_token_idx: LxRoseTokenIdx,
        environment_signature: LxEnvironmentSignature,
        stmts: VdSemStmtIdxRange,
        end_rcurl_token_idx: LxRoseTokenIdx,
    },
}

pub type VdSemStmtArena = Arena<VdSemStmtData>;
pub type VdSemStmtArenaRef<'a> = ArenaRef<'a, VdSemStmtData>;
pub type VdSemStmtIdx = ArenaIdx<VdSemStmtData>;
pub type VdSemStmtIdxRange = ArenaIdxRange<VdSemStmtData>;
pub type VdSemStmtMap<T> = ArenaMap<VdSemStmtData, T>;
pub type VdSemStmtOrderedMap<T> = ArenaOrderedMap<VdSemStmtData, T>;

impl ToVdSem<VdSemStmtIdxRange> for VdSynStmtIdxRange {
    // there is no need to cache because stmts will be created in one go
    fn to_vd_sem(self, builder: &mut VdSemExprBuilder) -> VdSemStmtIdxRange {
        let mut stmts: Vec<VdSemStmtData> = vec![];
        for stmt in self {
            stmts.push(builder.build_stmt(stmt));
        }
        builder.alloc_stmts(stmts)
    }
}

impl<'a> VdSemExprBuilder<'a> {
    pub(crate) fn build_stmt(&mut self, stmt: VdSynStmtIdx) -> VdSemStmtData {
        match self.syn_stmt_arena()[stmt] {
            VdSynStmtData::Paragraph(sentences) => {
                VdSemStmtData::Paragraph(sentences.to_vd_sem(self))
            }
            VdSynStmtData::Environment {
                environment_signature,
                stmts,
                begin_command_token_idx,
                end_rcurl_token_idx,
            } => VdSemStmtData::Environment {
                environment_signature,
                stmts: stmts.to_vd_sem(self),
                begin_command_token_idx,
                end_rcurl_token_idx,
            },
        }
    }
}

pub enum VdSemStmtChild {
    Sentence(VdSemSentenceIdx),
    Stmt(VdSemStmtIdx),
}

impl VdSemStmtData {
    pub fn children(&self) -> Vec<VdSemStmtChild> {
        match *self {
            VdSemStmtData::Paragraph(sentences) => sentences
                .into_iter()
                .map(|s| VdSemStmtChild::Sentence(s))
                .collect(),
            VdSemStmtData::Environment {
                environment_signature,
                stmts,
                begin_command_token_idx,
                end_rcurl_token_idx,
            } => stmts.into_iter().map(|s| VdSemStmtChild::Stmt(s)).collect(),
        }
    }
}
