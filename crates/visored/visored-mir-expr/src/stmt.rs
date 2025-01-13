//! Note the difference of the concept of `stmt` in visored compared with those in provers like Lean, Rocq, Isabelle.
//!
//! Things like `have` and `show` are tactics in traditional provers, but are statements in visored.
//!
//! This is actually more similar to those in natural languages.
pub(crate) mod batch;
pub mod block;
#[cfg(test)]
mod tests;

use self::{batch::*, block::*};
use crate::{expr::VdMirExprIdx, pattern::VdMirPattern, *};
use hint::{VdMirHintData, VdMirHintEntry, VdMirHintIdx, VdMirHintIdxRange, VdMirHintSource};
use hypothesis::{contradiction::VdMirHypothesisResult, VdMirHypothesisIdx};
use idx_arena::{
    map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef,
};
use once_place::OncePlace;
use visored_entity_path::module::VdModulePath;
use visored_global_resolution::resolution::environment::VdEnvironmentGlobalResolution;
use visored_prelude::division::VdDivisionLevel;
use visored_sem_expr::{
    block::{VdSemBlockData, VdSemBlockIdx, VdSemBlockIdxRange},
    clause::{
        r#let::{placeholder::VdSemLetClausePlaceholderTypeRepr, VdSemLetClauseDispatch},
        VdSemClauseData, VdSemClauseIdx, VdSemClauseIdxRange,
    },
    division::{VdSemDivisionData, VdSemDivisionIdx, VdSemDivisionIdxRange},
    sentence::{VdSemSentenceData, VdSemSentenceIdx, VdSemSentenceIdxRange},
};
use visored_term::ty::VdType;

#[derive(Debug, PartialEq, Eq)]
pub enum VdMirStmtData {
    Block {
        blocks: VdMirStmtIdxRange,
        meta: VdMirBlockMeta,
    },
    LetAssigned {
        pattern: VdMirPattern,
        assignment: VdMirExprIdx,
        hypothesis_chunk_place: OncePlace<VdMirHypothesisResult>,
    },
    LetPlaceholder {
        pattern: VdMirPattern,
        ty: VdMirExprIdx,
    },
    Assume {
        prop: VdMirExprIdx,
        hypothesis_chunk_place: OncePlace<VdMirHypothesisResult>,
    },
    Goal {
        prop: VdMirExprIdx,
    },
    Have {
        prop: VdMirExprIdx,
        hint: Option<VdMirHintIdx>,
        hypothesis_chunk_place: OncePlace<VdMirHypothesisResult>,
    },
    Show {
        prop: VdMirExprIdx,
        hint: Option<VdMirHintIdx>,
        goal_and_hypothesis_chunk_place: Option<(VdMirExprIdx, OncePlace<VdMirHypothesisResult>)>,
    },
    Qed {
        goal_and_hypothesis_chunk_place: Option<(VdMirExprIdx, OncePlace<VdMirHypothesisResult>)>,
    },
}

#[derive(Debug)]
pub struct VdMirStmtEntry {
    data: VdMirStmtData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VdMirStmtSource {
    Stmt(VdSemBlockIdx),
    Division(VdSemDivisionIdx),
    Sentence(VdSemSentenceIdx),
    Clause(VdSemClauseIdx),
    Qed(VdSemSentenceIdxRange),
}

pub type VdMirStmtArena = Arena<VdMirStmtEntry>;
pub type VdMirStmtOrderedMap<T> = ArenaOrderedMap<VdMirStmtEntry, T>;
pub type VdMirStmtMap<T> = ArenaMap<VdMirStmtEntry, T>;
pub type VdMirStmtArenaRef<'a> = ArenaRef<'a, VdMirStmtEntry>;
pub type VdMirStmtIdx = ArenaIdx<VdMirStmtEntry>;
pub type VdMirStmtIdxRange = ArenaIdxRange<VdMirStmtEntry>;

impl VdMirStmtEntry {
    pub fn new(data: VdMirStmtData) -> Self {
        Self { data }
    }

    pub fn new_qed(goal: Option<VdMirExprIdx>) -> Self {
        Self::new(VdMirStmtData::Qed {
            goal_and_hypothesis_chunk_place: goal.map(|goal| (goal, OncePlace::default())),
        })
    }
}

impl VdMirStmtEntry {
    pub fn data(&self) -> &VdMirStmtData {
        &self.data
    }

    pub(crate) fn data_mut(&mut self) -> &mut VdMirStmtData {
        &mut self.data
    }
}

impl<'db> ToVdMir<VdMirStmtIdxRange, VdMirExprBuilder<'db>> for VdSemDivisionIdxRange {
    fn to_vd_mir(self, builder: &mut VdMirExprBuilder<'db>) -> VdMirStmtIdxRange {
        let mut stmt_batch = VdMirStmtBatch::new();
        for division in self {
            stmt_batch.push(
                VdMirStmtEntry::new(builder.build_stmt_from_sem_division(division)),
                VdMirStmtSource::Division(division),
            );
        }
        builder.alloc_stmts(stmt_batch)
    }
}

impl<'db> VdMirExprBuilder<'db> {
    fn build_stmt_from_sem_division(&mut self, division: VdSemDivisionIdx) -> VdMirStmtData {
        match *self.sem_division_arena()[division].data() {
            VdSemDivisionData::Blocks { blocks } => VdMirStmtData::Block {
                blocks: blocks.to_vd_mir(self),
                meta: VdMirBlockMeta::Division(
                    VdDivisionLevel::Blocks,
                    self.sem_division_arena()[division].module_path(),
                ),
            },
            // TODO: what to do for title?
            VdSemDivisionData::Divisions {
                command_token_idx,
                level,
                lcurl_token_idx,
                rcurl_token_idx,
                subdivisions,
            } => VdMirStmtData::Block {
                blocks: subdivisions.to_vd_mir(self),
                meta: VdMirBlockMeta::Division(
                    level,
                    self.sem_division_arena()[division].module_path(),
                ),
            },
        }
    }
}

impl<'db> ToVdMir<VdMirStmtIdxRange, VdMirExprBuilder<'db>> for VdSemBlockIdxRange {
    fn to_vd_mir(self, builder: &mut VdMirExprBuilder<'db>) -> VdMirStmtIdxRange {
        let mut stmt_batch = VdMirStmtBatch::new();
        for stmt in self {
            builder.build_mir_stmts_from_block(stmt, &mut stmt_batch);
        }
        builder.alloc_stmts(stmt_batch)
    }
}

impl<'db> VdMirExprBuilder<'db> {
    fn build_mir_stmts_from_block(
        &mut self,
        block: VdSemBlockIdx,
        stmt_batch: &mut VdMirStmtBatch,
    ) {
        match *self.sem_block_arena()[block].data() {
            VdSemBlockData::Paragraph(sentences) => {
                self.build_stmts_from_sem_sentences(sentences, stmt_batch)
            }
            VdSemBlockData::Environment {
                environment_signature,
                resolution,
                blocks,
                begin_command_token_idx,
                end_rcurl_token_idx,
            } => todo!(),
        }
    }
}

impl<'db> VdMirExprBuilder<'db> {
    fn build_stmts_from_sem_sentences(
        &mut self,
        sentences: VdSemSentenceIdxRange,
        stmt_batch: &mut VdMirStmtBatch,
    ) {
        for sentence in sentences {
            self.build_stmts_from_sem_sentence(sentence, stmt_batch);
        }
    }

    fn build_stmts_from_sem_sentence(
        &mut self,
        sentence: VdSemSentenceIdx,
        stmt_batch: &mut VdMirStmtBatch,
    ) {
        match self.sem_sentence_arena()[sentence] {
            VdSemSentenceData::Clauses { clauses, end } => {
                self.build_stmts_from_sem_clauses(clauses, stmt_batch);
            }
            VdSemSentenceData::Have => todo!(),
            VdSemSentenceData::Show => todo!(),
        }
    }
}

impl<'db> VdMirExprBuilder<'db> {
    fn build_stmts_from_sem_clauses(
        &mut self,
        clauses: VdSemClauseIdxRange,
        stmt_batch: &mut VdMirStmtBatch,
    ) {
        for clause in clauses {
            let stmt_data = self.build_stmt_from_sem_clause(clause, stmt_batch.goal_place_mut());
            stmt_batch.push(
                VdMirStmtEntry::new(stmt_data),
                VdMirStmtSource::Clause(clause),
            );
        }
    }

    fn build_stmt_from_sem_clause(
        &mut self,
        clause: VdSemClauseIdx,
        goal: &mut OncePlace<VdMirExprIdx>,
    ) -> VdMirStmtData {
        match *self.sem_clause_arena()[clause].data() {
            VdSemClauseData::Verb => todo!(),
            VdSemClauseData::Let {
                left_math_delimiter_token_idx: left_dollar_token_idx,
                formula,
                right_math_delimiter_token_idx: right_dollar_token_idx,
                ref dispatch,
            } => match dispatch {
                VdSemLetClauseDispatch::Assigned(dispatch) => VdMirStmtData::LetAssigned {
                    pattern: dispatch.pattern().to_vd_mir(self),
                    assignment: dispatch.assignment().to_vd_mir(self),
                    hypothesis_chunk_place: OncePlace::default(),
                },
                VdSemLetClauseDispatch::Placeholder(dispatch) => VdMirStmtData::LetPlaceholder {
                    pattern: dispatch.pattern().to_vd_mir(self),
                    ty: match *dispatch.ty_repr() {
                        VdSemLetClausePlaceholderTypeRepr::Expr(ty_expr) => ty_expr.to_vd_mir(self),
                    },
                },
            },
            VdSemClauseData::Assume { formula, .. } => VdMirStmtData::Assume {
                prop: formula.to_vd_mir(self),
                hypothesis_chunk_place: OncePlace::default(),
            },
            VdSemClauseData::Goal { formula, .. } => {
                let prop = formula.to_vd_mir(self);
                goal.set(prop);
                VdMirStmtData::Goal { prop }
            }
            VdSemClauseData::Have { formula, .. } => VdMirStmtData::Have {
                prop: formula.to_vd_mir(self),
                hint: None, // ad hoc
                hypothesis_chunk_place: OncePlace::default(),
            },
            VdSemClauseData::Show { formula, .. } => VdMirStmtData::Show {
                prop: formula.to_vd_mir(self),
                hint: None,
                goal_and_hypothesis_chunk_place: match goal.get().cloned() {
                    Some(goal) => Some((goal, OncePlace::default())),
                    None => None,
                },
            },
            VdSemClauseData::Todo(lx_rose_token_idx) => todo!(),
        }
    }
}
