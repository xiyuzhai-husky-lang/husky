//! Note the difference of the concept of `stmt` in visored compared with those in provers like Lean, Rocq, Isabelle.
//!
//! Things like `have` and `show` are tactics in traditional provers, but are statements in visored.
//!
//! This is actually more similar to those in natural languages.
pub mod block;
#[cfg(test)]
mod tests;

use self::block::*;
use crate::{expr::VdMirExprIdx, pattern::VdMirPattern, *};
use idx_arena::{
    map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef,
};
use tactic::{VdMirTacticData, VdMirTacticEntry, VdMirTacticIdxRange, VdMirTacticSource};
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
        stmts: VdMirStmtIdxRange,
        meta: VdMirBlockMeta,
    },
    LetPlaceholder {
        pattern: VdMirPattern,
        ty: VdMirExprIdx,
    },
    LetAssigned {
        pattern: VdMirPattern,
        assignment: VdMirExprIdx,
    },
    Goal {
        prop: VdMirExprIdx,
    },
    Have {
        prop: VdMirExprIdx,
        tactics: VdMirTacticIdxRange,
    },
    Show {
        prop: VdMirExprIdx,
        tactics: VdMirTacticIdxRange,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VdMirStmtSource {
    Stmt(VdSemBlockIdx),
    Division(VdSemDivisionIdx),
    Sentence(VdSemSentenceIdx),
    Clause(VdSemClauseIdx),
}

pub type VdMirStmtArena = Arena<VdMirStmtData>;
pub type VdMirStmtOrderedMap<T> = ArenaOrderedMap<VdMirStmtData, T>;
pub type VdMirStmtMap<T> = ArenaMap<VdMirStmtData, T>;
pub type VdMirStmtArenaRef<'a> = ArenaRef<'a, VdMirStmtData>;
pub type VdMirStmtIdx = ArenaIdx<VdMirStmtData>;
pub type VdMirStmtIdxRange = ArenaIdxRange<VdMirStmtData>;

impl ToVdMir<VdMirStmtIdxRange> for VdSemDivisionIdxRange {
    fn to_vd_mir(self, builder: &mut VdMirExprBuilder) -> VdMirStmtIdxRange {
        let data = self
            .into_iter()
            .map(|division| builder.build_stmt_from_sem_division(division))
            .collect::<Vec<_>>();
        let sources = self.into_iter().map(VdMirStmtSource::Division);
        builder.alloc_stmts(data, sources)
    }
}

impl<'db> VdMirExprBuilder<'db> {
    fn build_stmt_from_sem_division(&mut self, division: VdSemDivisionIdx) -> VdMirStmtData {
        match *self.sem_division_arena()[division].data() {
            VdSemDivisionData::Stmts { stmts } => VdMirStmtData::Block {
                stmts: stmts.to_vd_mir(self),
                meta: VdMirBlockMeta::Division(
                    VdDivisionLevel::Stmts,
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
                stmts: subdivisions.to_vd_mir(self),
                meta: VdMirBlockMeta::Division(
                    level,
                    self.sem_division_arena()[division].module_path(),
                ),
            },
        }
    }
}

impl ToVdMir<VdMirStmtIdxRange> for VdSemBlockIdxRange {
    fn to_vd_mir(self, builder: &mut VdMirExprBuilder) -> VdMirStmtIdxRange {
        let data = self
            .into_iter()
            .map(|stmt| builder.build_stmt_from_sem_stmt(stmt))
            .collect::<Vec<_>>();
        let sources = self.into_iter().map(VdMirStmtSource::Stmt);
        builder.alloc_stmts(data, sources)
    }
}

impl<'db> VdMirExprBuilder<'db> {
    fn build_stmt_from_sem_stmt(&mut self, stmt: VdSemBlockIdx) -> VdMirStmtData {
        match *self.sem_stmt_arena()[stmt].data() {
            VdSemBlockData::Paragraph(sentences) => VdMirStmtData::Block {
                stmts: sentences.to_vd_mir(self),
                meta: VdMirBlockMeta::Paragraph,
            },
            VdSemBlockData::Environment {
                environment_signature,
                resolution,
                stmts,
                begin_command_token_idx,
                end_rcurl_token_idx,
            } => VdMirStmtData::Block {
                stmts: stmts.to_vd_mir(self),
                meta: VdMirBlockMeta::Environment(
                    environment_signature.path(),
                    match resolution {
                        VdEnvironmentGlobalResolution::Environment(vd_environment_path) => {
                            vd_environment_path
                        }
                    },
                    self.sem_stmt_arena()[stmt].module_path(),
                ),
            },
        }
    }
}

impl ToVdMir<VdMirStmtIdxRange> for VdSemSentenceIdxRange {
    fn to_vd_mir(self, builder: &mut VdMirExprBuilder) -> VdMirStmtIdxRange {
        let data = self
            .into_iter()
            .map(|sentence| builder.build_stmt_from_sem_sentence(sentence))
            .collect::<Vec<_>>();
        let sources = self.into_iter().map(VdMirStmtSource::Sentence);
        builder.alloc_stmts(data, sources)
    }
}

impl<'db> VdMirExprBuilder<'db> {
    fn build_stmt_from_sem_sentence(&mut self, sentence: VdSemSentenceIdx) -> VdMirStmtData {
        match self.sem_sentence_arena()[sentence] {
            VdSemSentenceData::Clauses { clauses, end } => VdMirStmtData::Block {
                stmts: clauses.to_vd_mir(self),
                meta: VdMirBlockMeta::Sentence,
            },
            VdSemSentenceData::Have => todo!(),
            VdSemSentenceData::Show => todo!(),
        }
    }
}

impl ToVdMir<VdMirStmtIdxRange> for VdSemClauseIdxRange {
    fn to_vd_mir(self, builder: &mut VdMirExprBuilder) -> VdMirStmtIdxRange {
        let data = self
            .into_iter()
            .map(|clause| builder.build_stmt_from_sem_clause(clause))
            .collect::<Vec<_>>();
        let sources = self.into_iter().map(VdMirStmtSource::Clause);
        builder.alloc_stmts(data, sources)
    }
}

impl<'db> VdMirExprBuilder<'db> {
    fn build_stmt_from_sem_clause(&mut self, clause: VdSemClauseIdx) -> VdMirStmtData {
        match *self.sem_clause_arena()[clause].data() {
            VdSemClauseData::Verb => todo!(),
            VdSemClauseData::Let {
                left_math_delimiter_token_idx: left_dollar_token_idx,
                formula,
                right_math_delimiter_token_idx: right_dollar_token_idx,
                ref dispatch,
            } => match dispatch {
                VdSemLetClauseDispatch::Assigned(dispatch) => todo!(),
                VdSemLetClauseDispatch::Placeholder(dispatch) => VdMirStmtData::LetPlaceholder {
                    pattern: dispatch.pattern().to_vd_mir(self),
                    ty: match *dispatch.ty_repr() {
                        VdSemLetClausePlaceholderTypeRepr::Expr(ty_expr) => ty_expr.to_vd_mir(self),
                    },
                },
            },
            VdSemClauseData::Assume {
                left_math_delimiter_token_idx: left_dollar_token_idx,
                formula,
                right_math_delimiter_token_idx: right_dollar_token_idx,
            } => VdMirStmtData::LetPlaceholder {
                pattern: VdMirPattern::Assumed,
                ty: formula.to_vd_mir(self),
            },
            VdSemClauseData::Goal {
                left_math_delimiter_token_idx: left_dollar_token_idx,
                formula,
                right_math_delimiter_token_idx: right_dollar_token_idx,
            } => VdMirStmtData::Goal {
                prop: formula.to_vd_mir(self),
            },
            VdSemClauseData::Have {
                left_math_delimiter_token_idx: left_dollar_token_idx,
                formula,
                right_math_delimiter_token_idx: right_dollar_token_idx,
            } => VdMirStmtData::Have {
                prop: formula.to_vd_mir(self),
                tactics: self.default_tactics(clause),
            },
            VdSemClauseData::Show {
                left_math_delimiter_token_idx: left_dollar_token_idx,
                formula,
                right_math_delimiter_token_idx: right_dollar_token_idx,
            } => VdMirStmtData::Show {
                prop: formula.to_vd_mir(self),
                tactics: self.default_tactics(clause),
            },
            VdSemClauseData::Todo(lx_rose_token_idx) => todo!(),
        }
    }

    fn default_tactics(&mut self, source: impl Into<VdMirTacticSource>) -> VdMirTacticIdxRange {
        self.alloc_tactics(
            [VdMirTacticEntry::new(VdMirTacticData::Obvious)],
            [source.into()],
        )
    }
}
