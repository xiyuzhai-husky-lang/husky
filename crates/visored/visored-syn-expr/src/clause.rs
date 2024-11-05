use std::iter::Peekable;

use super::*;
use builder::VdSynExprBuilder;
use expr::VdSynExprIdx;
use husky_coword::Coword;
use idx_arena::{
    map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef,
};
use latex_ast::ast::{
    math::{LxMathAstIdx, LxMathAstIdxRange},
    rose::{LxRoseAstData, LxRoseAstIdx},
};
use latex_token::idx::LxRoseTokenIdx;

// TODO: this is just an ad hoc placeholder implementation
pub enum VdSynClauseData {
    Let {
        let_token_idx: LxRoseTokenIdx,
        left_dollar_token_idx: LxRoseTokenIdx,
        math_asts: LxMathAstIdxRange,
        right_dollar_token_idx: LxRoseTokenIdx,
    },
    Assume {
        assume_token_idx: LxRoseTokenIdx,
        left_dollar_token_idx: LxRoseTokenIdx,
        math_asts: LxMathAstIdxRange,
        right_dollar_token_idx: LxRoseTokenIdx,
    },
}

pub enum VdSynClauseChild {
    Expr(VdSynExprIdx),
}

impl VdSynClauseData {
    pub(crate) fn children(&self) -> Vec<VdSynClauseChild> {
        todo!()
    }
}

pub type VdSynClauseArena = Arena<VdSynClauseData>;
pub type VdSynClauseArenaRef<'a> = ArenaRef<'a, VdSynClauseData>;
pub type VdSynClauseIdx = ArenaIdx<VdSynClauseData>;
pub type VdSynClauseIdxRange = ArenaIdxRange<VdSynClauseData>;
pub type VdSynClauseMap<T> = ArenaMap<VdSynClauseData, T>;
pub type VdSynClauseOrderedMap<T> = ArenaOrderedMap<VdSynClauseData, T>;

impl<'db> VdSynExprBuilder<'db> {
    pub(crate) fn parse_clause(
        &mut self,
        token_idx: LxRoseTokenIdx,
        word: Coword,
        asts: &mut Peekable<impl Iterator<Item = LxRoseAstIdx>>,
    ) -> VdSynClauseData {
        match word.data(self.db()) {
            "Let" | "let" => {
                let ast = asts.next().expect("expect a let clause");
                match self.ast_arena()[ast] {
                    LxRoseAstData::Math {
                        left_dollar_token_idx,
                        math_asts,
                        right_dollar_token_idx,
                    } => VdSynClauseData::Let {
                        let_token_idx: token_idx,
                        left_dollar_token_idx,
                        math_asts,
                        right_dollar_token_idx,
                    },
                    LxRoseAstData::TextEdit { ref buffer } => todo!(),
                    LxRoseAstData::Word(lx_rose_token_idx, coword) => todo!(),
                    LxRoseAstData::Punctuation(lx_rose_token_idx, lx_rose_punctuation) => todo!(),
                }
            }
            "Assume" | "assume" | "Suppose" | "suppose" => {
                let ast = asts.next().expect("expect a assume clause");
                match self.ast_arena()[ast] {
                    LxRoseAstData::Math {
                        left_dollar_token_idx,
                        math_asts,
                        right_dollar_token_idx,
                    } => VdSynClauseData::Assume {
                        assume_token_idx: token_idx,
                        left_dollar_token_idx,
                        math_asts,
                        right_dollar_token_idx,
                    },
                    LxRoseAstData::TextEdit { ref buffer } => todo!(),
                    LxRoseAstData::Word(lx_rose_token_idx, coword) => todo!(),
                    LxRoseAstData::Punctuation(lx_rose_token_idx, lx_rose_punctuation) => todo!(),
                }
            }
            _ => todo!(),
        }
    }
}
