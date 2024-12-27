pub mod helpers;
pub mod r#let;
#[cfg(test)]
mod tests;

use self::r#let::*;
use super::*;
use base_coword::BaseCoword;
use builder::VdSynExprBuilder;
use expr::VdSynExprIdx;
use idx_arena::{
    map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef,
};
use latex_ast::ast::{
    math::{LxMathAstIdx, LxMathAstIdxRange},
    rose::{LxRoseAstData, LxRoseAstIdx},
};
use latex_token::idx::LxRoseTokenIdx;
use once_place::OncePlace;
use sentence::cnl::CnlToken;
use std::iter::Peekable;
use symbol::builder::VdSynSymbolBuilder;
use vibe::VdSynExprVibe;

#[derive(Debug, PartialEq, Eq)]
pub enum VdSynClauseData {
    Let {
        left_math_delimiter_token_idx: LxRoseTokenIdx,
        formula: VdSynExprIdx,
        right_math_delimiter_token_idx: LxRoseTokenIdx,
    },
    Assume {
        left_math_delimiter_token_idx: LxRoseTokenIdx,
        formula: VdSynExprIdx,
        right_math_delimiter_token_idx: LxRoseTokenIdx,
    },
    Have {
        left_math_delimiter_token_idx: LxRoseTokenIdx,
        formula: VdSynExprIdx,
        right_math_delimiter_token_idx: LxRoseTokenIdx,
    },
    Show {
        left_math_delimiter_token_idx: LxRoseTokenIdx,
        formula: VdSynExprIdx,
        right_math_delimiter_token_idx: LxRoseTokenIdx,
    },
    Goal {
        left_math_delimiter_token_idx: LxRoseTokenIdx,
        formula: ArenaIdx<crate::expr::VdSynExprData>,
        right_math_delimiter_token_idx: LxRoseTokenIdx,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum VdSynClauseEntry {
    Cnl {
        tokens: Vec<CnlToken>,
        data: VdSynClauseData,
    },
    Unl {
        tokens: (),
        data: OncePlace<VdSynClauseData>,
    },
}

pub type VdSynClauseArena = Arena<VdSynClauseEntry>;
pub type VdSynClauseArenaRef<'a> = ArenaRef<'a, VdSynClauseEntry>;
pub type VdSynClauseIdx = ArenaIdx<VdSynClauseEntry>;
pub type VdSynClauseIdxRange = ArenaIdxRange<VdSynClauseEntry>;
pub type VdSynClauseMap<T> = ArenaMap<VdSynClauseEntry, T>;
pub type VdSynClauseOrderedMap<T> = ArenaOrderedMap<VdSynClauseEntry, T>;

impl VdSynClauseEntry {
    pub fn data(&self) -> &VdSynClauseData {
        match self {
            VdSynClauseEntry::Cnl { data, .. } => data,
            VdSynClauseEntry::Unl { data, .. } => data,
        }
    }

    #[track_caller]
    pub fn cnl_tokens(&self) -> &[CnlToken] {
        match self {
            VdSynClauseEntry::Cnl { tokens, .. } => tokens,
            VdSynClauseEntry::Unl { .. } => unreachable!(),
        }
    }
}

impl<'db> VdSynExprBuilder<'db> {
    pub(crate) fn parse_clause(
        &mut self,
        token_idx: LxRoseTokenIdx,
        word: BaseCoword,
        asts: &mut Peekable<impl Iterator<Item = LxRoseAstIdx>>,
        vibe: VdSynExprVibe,
    ) -> VdSynClauseData {
        let db = self.db();
        match word.data() {
            "Let" | "let" => {
                let ast = asts.next().expect("expect a let clause");
                match self.ast_arena()[ast] {
                    LxRoseAstData::Math {
                        left_delimiter_token_idx,
                        math_asts,
                        right_delimiter_token_idx,
                    } => {
                        let formula = (
                            ((*left_delimiter_token_idx + 1)..*right_delimiter_token_idx).into(),
                            math_asts,
                        )
                            .to_vd_syn(self, vibe);
                        VdSynClauseData::Let {
                            left_math_delimiter_token_idx: left_delimiter_token_idx,
                            formula,
                            right_math_delimiter_token_idx: right_delimiter_token_idx,
                        }
                    }
                    LxRoseAstData::TextEdit { ref buffer } => todo!(),
                    LxRoseAstData::Word(lx_rose_token_idx, coword) => todo!(),
                    LxRoseAstData::Punctuation(lx_rose_token_idx, lx_rose_punctuation) => todo!(),
                    LxRoseAstData::Delimited {
                        left_delimiter_token_idx,
                        left_delimiter,
                        asts,
                        right_delimiter_token_idx,
                        right_delimiter,
                    } => todo!(),
                    LxRoseAstData::CompleteCommand {
                        command_token_idx,
                        command_path,
                        options,
                        ref arguments,
                    } => todo!(),
                    LxRoseAstData::Environment { .. } => todo!(),
                    LxRoseAstData::NewParagraph(_) => todo!(),
                }
            }
            "Assume" | "assume" | "Suppose" | "suppose" => {
                let ast = asts.next().expect("expect a assume clause");
                match self.ast_arena()[ast] {
                    LxRoseAstData::Math {
                        left_delimiter_token_idx: left_dollar_token_idx,
                        math_asts,
                        right_delimiter_token_idx: right_dollar_token_idx,
                    } => VdSynClauseData::Assume {
                        left_math_delimiter_token_idx: left_dollar_token_idx,
                        formula: (
                            ((*left_dollar_token_idx + 1)..*right_dollar_token_idx).into(),
                            math_asts,
                        )
                            .to_vd_syn(self, vibe),
                        right_math_delimiter_token_idx: right_dollar_token_idx,
                    },
                    LxRoseAstData::TextEdit { ref buffer } => todo!(),
                    LxRoseAstData::Word(lx_rose_token_idx, coword) => todo!(),
                    LxRoseAstData::Punctuation(lx_rose_token_idx, lx_rose_punctuation) => todo!(),
                    LxRoseAstData::Delimited {
                        left_delimiter_token_idx,
                        left_delimiter,
                        asts,
                        right_delimiter_token_idx,
                        right_delimiter,
                    } => todo!(),
                    LxRoseAstData::CompleteCommand {
                        command_token_idx,
                        command_path,
                        options,
                        ref arguments,
                    } => todo!(),
                    LxRoseAstData::Environment { .. } => todo!(),
                    LxRoseAstData::NewParagraph(_) => todo!(),
                }
            }
            "Then" | "then" => {
                let ast = asts.next().expect("expect a then clause");
                match self.ast_arena()[ast] {
                    LxRoseAstData::Math {
                        left_delimiter_token_idx: left_dollar_token_idx,
                        math_asts,
                        right_delimiter_token_idx: right_dollar_token_idx,
                    } => VdSynClauseData::Have {
                        left_math_delimiter_token_idx: left_dollar_token_idx,
                        formula: (
                            ((*left_dollar_token_idx + 1)..*right_dollar_token_idx).into(),
                            math_asts,
                        )
                            .to_vd_syn(self, vibe),
                        right_math_delimiter_token_idx: right_dollar_token_idx,
                    },
                    LxRoseAstData::TextEdit { ref buffer } => todo!(),
                    LxRoseAstData::Word(lx_rose_token_idx, coword) => todo!(),
                    LxRoseAstData::Punctuation(lx_rose_token_idx, lx_rose_punctuation) => todo!(),
                    LxRoseAstData::Delimited {
                        left_delimiter_token_idx,
                        left_delimiter,
                        asts,
                        right_delimiter_token_idx,
                        right_delimiter,
                    } => todo!(),
                    LxRoseAstData::CompleteCommand {
                        command_token_idx,
                        command_path,
                        options,
                        ref arguments,
                    } => todo!(),
                    LxRoseAstData::Environment { .. } => todo!(),
                    LxRoseAstData::NewParagraph(_) => todo!(),
                }
            }
            _ => todo!(), // VdSynClauseData::Todo(token_idx),
        }
    }
}

impl<'db> VdSynSymbolBuilder<'db> {
    pub(crate) fn build_clause_aux(&mut self, clause: VdSynClauseIdx) {
        match *self.clause_arena()[clause].data() {
            VdSynClauseData::Let { formula, .. } => {
                let resolution = self.infer_let_clause_resolution(clause, formula);
                self.build_symbols_in_let_resolution(clause, resolution)
            }
            VdSynClauseData::Assume { formula, .. } => self.build_expr(formula),
            VdSynClauseData::Goal { formula, .. } => self.build_expr(formula),
            VdSynClauseData::Have { formula, .. } => self.build_expr(formula),
            VdSynClauseData::Show { formula, .. } => self.build_expr(formula),
        }
    }

    pub(crate) fn build_symbols_in_let_resolution(
        &mut self,
        clause: VdSynClauseIdx,
        resolution: VdSynLetClauseResolution,
    ) {
        match resolution {
            VdSynLetClauseResolution::Assigned(resolution) => {
                self.build_symbols_in_let_assigned_resolution(clause, resolution)
            }
            VdSynLetClauseResolution::Placeholder(resolution) => {
                self.build_symbols_in_let_placeholder_resolution(clause, resolution)
            }
        }
    }
}
