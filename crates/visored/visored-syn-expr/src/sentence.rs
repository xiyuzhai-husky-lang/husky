mod cnl;
pub mod helpers;
mod unl;

use crate::{
    builder::{ToVdSyn, VdSynExprBuilder},
    clause::{VdSynClauseIdx, VdSynClauseIdxRange},
    expr::VdSynExprIdx,
    symbol::builder::VdSynSymbolBuilder,
    vibe::VdSynExprVibe,
};
use base_coword::BaseCoword;
use cnl::CnlToken;
use idx_arena::{
    map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef,
};
use latex_ast::ast::rose::{LxRoseAstData, LxRoseAstIdx, LxRoseAstIdxRange};
use latex_rose_punctuation::LxRosePunctuation;
use latex_token::idx::LxRoseTokenIdx;
use once_place::OncePlace;
use snl_prelude::mode::SnlMode;
use std::iter::Peekable;

#[derive(Debug, PartialEq, Eq)]
pub enum VdSynSentenceData {
    Have,
    Show,
    Clauses {
        clauses: VdSynClauseIdxRange,
        end: VdSynSentenceEnd,
    },
    Let {
        left_math_delimiter_token_idx: LxRoseTokenIdx,
        formula: VdSynExprIdx,
        right_math_delimiter_token_idx: LxRoseTokenIdx,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum VdSynSentenceEntry {
    Cnl {
        tokens: Vec<CnlToken>,
        data: VdSynSentenceData,
    },
    Unl {
        tokens: (),
        data: OncePlace<VdSynSentenceData>,
    },
}

impl std::ops::Deref for VdSynSentenceEntry {
    type Target = VdSynSentenceData;

    fn deref(&self) -> &Self::Target {
        self.data()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdSynSentenceEnd {
    Period(LxRoseTokenIdx),
    Void,
}

pub type VdSynSentenceArena = Arena<VdSynSentenceEntry>;
pub type VdSynSentenceArenaRef<'a> = ArenaRef<'a, VdSynSentenceEntry>;
pub type VdSynSentenceIdx = ArenaIdx<VdSynSentenceEntry>;
pub type VdSynSentenceIdxRange = ArenaIdxRange<VdSynSentenceEntry>;
pub type VdSynSentenceMap<T> = ArenaMap<VdSynSentenceEntry, T>;
pub type VdSynSentenceOrderedMap<T> = ArenaOrderedMap<VdSynSentenceEntry, T>;

impl VdSynSentenceEntry {
    pub fn data(&self) -> &VdSynSentenceData {
        match self {
            VdSynSentenceEntry::Cnl { data, .. } => data,
            VdSynSentenceEntry::Unl { data, .. } => data,
        }
    }

    #[track_caller]
    pub fn cnl_tokens(&self) -> &[CnlToken] {
        match self {
            VdSynSentenceEntry::Cnl { tokens, .. } => tokens,
            VdSynSentenceEntry::Unl { .. } => unreachable!(),
        }
    }
}

impl<'db> VdSynExprBuilder<'db> {
    pub(crate) fn parse_sentence(
        &mut self,
        asts: &mut Peekable<impl Iterator<Item = LxRoseAstIdx>>,
        vibe: VdSynExprVibe,
    ) -> VdSynSentenceEntry {
        match vibe.snl_mode() {
            SnlMode::Unl => todo!(),
            SnlMode::Cnl => self.parse_cnl_sentence(asts, vibe),
        }
    }
}

impl<'db> VdSynSymbolBuilder<'db> {
    pub(crate) fn build_sentence_aux(&mut self, sentence: VdSynSentenceIdx) {
        match *self.sentence_arena()[sentence].data() {
            VdSynSentenceData::Clauses { clauses, .. } => self.build_clauses(clauses),
            VdSynSentenceData::Have => todo!(),
            VdSynSentenceData::Show => todo!(),
            VdSynSentenceData::Let { formula, .. } => {
                todo!()
            }
        }
    }
}
