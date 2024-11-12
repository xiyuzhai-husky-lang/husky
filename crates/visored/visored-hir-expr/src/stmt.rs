pub mod block;

use self::block::*;
use crate::{expr::VdHirExprIdx, *};
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use visored_sem_expr::{
    clause::{VdSemClauseData, VdSemClauseIdx, VdSemClauseIdxRange},
    sentence::{VdSemSentenceData, VdSemSentenceIdx, VdSemSentenceIdxRange},
    stmt::{VdSemStmtData, VdSemStmtIdx, VdSemStmtIdxRange},
};

#[derive(Debug, PartialEq, Eq)]
pub enum VdHirStmtData {
    // Add appropriate variants here, for example:
    Expression(VdHirExprIdx),
    Block {
        stmts: VdHirStmtIdxRange,
        meta: VdHirBlockMeta,
    },
    Let,
    // Add more variants as needed
}

pub type VdHirStmtArena = Arena<VdHirStmtData>;
pub type VdHirStmtArenaRef<'a> = ArenaRef<'a, VdHirStmtData>;
pub type VdHirStmtIdx = ArenaIdx<VdHirStmtData>;
pub type VdHirStmtIdxRange = ArenaIdxRange<VdHirStmtData>;

impl ToVdHir<VdHirStmtIdxRange> for VdSemStmtIdxRange {
    fn to_vd_hir(self, builder: &mut VdHirExprBuilder) -> VdHirStmtIdxRange {
        let data = self
            .into_iter()
            .map(|stmt| builder.build_stmt_from_sem_stmt(stmt))
            .collect::<Vec<_>>();
        builder.alloc_stmts(data)
    }
}

impl<'db> VdHirExprBuilder<'db> {
    fn build_stmt_from_sem_stmt(&mut self, stmt: VdSemStmtIdx) -> VdHirStmtData {
        match self.sem_stmt_arena()[stmt] {
            VdSemStmtData::Paragraph(sentences) => VdHirStmtData::Block {
                stmts: sentences.to_vd_hir(self),
                meta: VdHirBlockMeta::Paragraph,
            },
            VdSemStmtData::Block { environment, stmts } => todo!(),
        }
    }
}

impl ToVdHir<VdHirStmtIdxRange> for VdSemSentenceIdxRange {
    fn to_vd_hir(self, builder: &mut VdHirExprBuilder) -> VdHirStmtIdxRange {
        let data = self
            .into_iter()
            .map(|sentence| builder.build_stmt_from_sem_sentence(sentence))
            .collect::<Vec<_>>();
        builder.alloc_stmts(data)
    }
}

impl<'db> VdHirExprBuilder<'db> {
    fn build_stmt_from_sem_sentence(&mut self, sentence: VdSemSentenceIdx) -> VdHirStmtData {
        match self.sem_sentence_arena()[sentence] {
            VdSemSentenceData::Clauses { clauses, end } => VdHirStmtData::Block {
                stmts: clauses.to_vd_hir(self),
                meta: VdHirBlockMeta::Sentence,
            },
        }
    }
}

impl ToVdHir<VdHirStmtIdxRange> for VdSemClauseIdxRange {
    fn to_vd_hir(self, builder: &mut VdHirExprBuilder) -> VdHirStmtIdxRange {
        let data = self
            .into_iter()
            .map(|clause| builder.build_stmt_from_sem_clause(clause))
            .collect::<Vec<_>>();
        builder.alloc_stmts(data)
    }
}

impl<'db> VdHirExprBuilder<'db> {
    fn build_stmt_from_sem_clause(&mut self, clause: VdSemClauseIdx) -> VdHirStmtData {
        match self.sem_clause_arena()[clause] {
            VdSemClauseData::Verb => todo!(),
            VdSemClauseData::Let {
                let_token_idx,
                left_dollar_token_idx,
                formula,
                right_dollar_token_idx,
                ref dispatch,
            } => VdHirStmtData::Let,
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
            } => todo!(),
        }
    }
}
