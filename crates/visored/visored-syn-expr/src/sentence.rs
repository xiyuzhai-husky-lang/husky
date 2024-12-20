mod cnl;
pub mod helpers;
mod unl;

use crate::{
    builder::{ToVdSyn, VdSynExprBuilder},
    clause::{VdSynClauseIdx, VdSynClauseIdxRange},
    vibe::VdSynExprVibe,
};
use base_coword::BaseCoword;
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
    Clauses {
        clauses: VdSynClauseIdxRange,
        end: VdSynSentenceEnd,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum VdSynSentenceEntry {
    Cnl {
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
            VdSynSentenceEntry::Cnl { data } => data,
            VdSynSentenceEntry::Unl { data, .. } => data,
        }
    }
}

impl<'db> VdSynExprBuilder<'db> {
    pub(crate) fn parse_sentence(
        &mut self,
        token_idx: LxRoseTokenIdx,
        word: BaseCoword,
        asts: &mut Peekable<impl Iterator<Item = LxRoseAstIdx>>,
        vibe: VdSynExprVibe,
    ) -> VdSynSentenceEntry {
        match vibe.snl_mode() {
            SnlMode::Unl => todo!(),
            SnlMode::Cnl => self.parse_cnl_sentence(token_idx, word, asts, vibe),
        }
    }
}
