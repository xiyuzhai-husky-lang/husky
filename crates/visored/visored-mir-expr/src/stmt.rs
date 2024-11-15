pub mod block;

use self::block::*;
use crate::{expr::VdMirExprIdx, pattern::VdMirPattern, *};
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use visored_sem_expr::{
    clause::{r#let::VdSemLetClauseDispatch, VdSemClauseData, VdSemClauseIdx, VdSemClauseIdxRange},
    sentence::{VdSemSentenceData, VdSemSentenceIdx, VdSemSentenceIdxRange},
    stmt::{VdSemStmtData, VdSemStmtIdx, VdSemStmtIdxRange},
};
use visored_zfc_ty::ty::VdZfcType;

#[derive(Debug, PartialEq, Eq)]
pub enum VdMirStmtData {
    Block {
        stmts: VdMirStmtIdxRange,
        meta: VdMirBlockMeta,
    },
    LetPlaceholder {
        pattern: VdMirPattern,
        ty: VdZfcType,
    },
    LetAssigned {
        pattern: VdMirPattern,
        assignment: VdMirExprIdx,
    },
    Then {
        formula: VdMirExprIdx,
    },
}

pub type VdMirStmtArena = Arena<VdMirStmtData>;
pub type VdMirStmtArenaRef<'a> = ArenaRef<'a, VdMirStmtData>;
pub type VdMirStmtIdx = ArenaIdx<VdMirStmtData>;
pub type VdMirStmtIdxRange = ArenaIdxRange<VdMirStmtData>;

impl ToVdMir<VdMirStmtIdxRange> for VdSemStmtIdxRange {
    fn to_vd_mir(self, builder: &mut VdMirExprBuilder) -> VdMirStmtIdxRange {
        let data = self
            .into_iter()
            .map(|stmt| builder.build_stmt_from_sem_stmt(stmt))
            .collect::<Vec<_>>();
        builder.alloc_stmts(data)
    }
}

impl<'db> VdMirExprBuilder<'db> {
    fn build_stmt_from_sem_stmt(&mut self, stmt: VdSemStmtIdx) -> VdMirStmtData {
        match self.sem_stmt_arena()[stmt] {
            VdSemStmtData::Paragraph(sentences) => VdMirStmtData::Block {
                stmts: sentences.to_vd_mir(self),
                meta: VdMirBlockMeta::Paragraph,
            },
            VdSemStmtData::Block { environment, stmts } => todo!(),
        }
    }
}

impl ToVdMir<VdMirStmtIdxRange> for VdSemSentenceIdxRange {
    fn to_vd_mir(self, builder: &mut VdMirExprBuilder) -> VdMirStmtIdxRange {
        let data = self
            .into_iter()
            .map(|sentence| builder.build_stmt_from_sem_sentence(sentence))
            .collect::<Vec<_>>();
        builder.alloc_stmts(data)
    }
}

impl<'db> VdMirExprBuilder<'db> {
    fn build_stmt_from_sem_sentence(&mut self, sentence: VdSemSentenceIdx) -> VdMirStmtData {
        match self.sem_sentence_arena()[sentence] {
            VdSemSentenceData::Clauses { clauses, end } => VdMirStmtData::Block {
                stmts: clauses.to_vd_mir(self),
                meta: VdMirBlockMeta::Sentence,
            },
        }
    }
}

impl ToVdMir<VdMirStmtIdxRange> for VdSemClauseIdxRange {
    fn to_vd_mir(self, builder: &mut VdMirExprBuilder) -> VdMirStmtIdxRange {
        let data = self
            .into_iter()
            .map(|clause| builder.build_stmt_from_sem_clause(clause))
            .collect::<Vec<_>>();
        builder.alloc_stmts(data)
    }
}

impl<'db> VdMirExprBuilder<'db> {
    fn build_stmt_from_sem_clause(&mut self, clause: VdSemClauseIdx) -> VdMirStmtData {
        match self.sem_clause_arena()[clause] {
            VdSemClauseData::Verb => todo!(),
            VdSemClauseData::Let {
                let_token_idx,
                left_dollar_token_idx,
                formula,
                right_dollar_token_idx,
                ref dispatch,
            } => match dispatch {
                VdSemLetClauseDispatch::Assigned(dispatch) => todo!(),
                VdSemLetClauseDispatch::Placeholder(dispatch) => VdMirStmtData::LetPlaceholder {
                    pattern: dispatch.pattern().to_vd_mir(self),
                    ty: dispatch.ty(),
                },
            },
            VdSemClauseData::Assume {
                assume_token_idx,
                left_dollar_token_idx,
                formula,
                right_dollar_token_idx,
            } => todo!(),
            VdSemClauseData::Then {
                then_token_idx,
                left_dollar_token_idx,
                formula,
                right_dollar_token_idx,
            } => VdMirStmtData::Then {
                formula: formula.to_vd_mir(self),
            },
        }
    }
}
