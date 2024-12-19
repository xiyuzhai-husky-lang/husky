pub mod helpers;

use std::iter::Peekable;

use crate::{
    builder::{ToVdSyn, VdSynExprBuilder},
    clause::{VdSynClauseIdx, VdSynClauseIdxRange},
};
use base_coword::BaseCoword;
use idx_arena::{
    map::ArenaMap, ordered_map::ArenaOrderedMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef,
};
use latex_ast::ast::rose::{LxRoseAstData, LxRoseAstIdx, LxRoseAstIdxRange};
use latex_rose_punctuation::LxRosePunctuation;
use latex_token::idx::LxRoseTokenIdx;

#[derive(Debug, PartialEq, Eq)]
pub enum VdSynSentenceData {
    Clauses {
        clauses: VdSynClauseIdxRange,
        end: VdSynSentenceEnd,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct VdSynSentenceEntry {
    data: VdSynSentenceData,
}

impl std::ops::Deref for VdSynSentenceEntry {
    type Target = VdSynSentenceData;

    fn deref(&self) -> &Self::Target {
        &self.data
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
        &self.data
    }
}

impl<'db> VdSynExprBuilder<'db> {
    pub(crate) fn parse_sentence(
        &mut self,
        token_idx: LxRoseTokenIdx,
        word: BaseCoword,
        asts: &mut Peekable<impl Iterator<Item = LxRoseAstIdx>>,
    ) -> VdSynSentenceEntry {
        let clauses = vec![self.parse_clause(token_idx, word, asts)];
        let end = loop {
            if self.peek_new_division(asts).is_some() {
                break VdSynSentenceEnd::Void;
            }
            if let Some(ast_idx) = asts.next() {
                match self.ast_arena()[ast_idx] {
                    LxRoseAstData::TextEdit { .. } => todo!(),
                    LxRoseAstData::Word(token_idx, coword) => {
                        self.emit_message_over_token_to_stdout(
                            *token_idx,
                            format!("coword: {}", coword),
                        );
                        todo!("coword: {}", coword)
                    }
                    LxRoseAstData::Punctuation(pucntuation_token_idx, punctuation) => {
                        match punctuation {
                            LxRosePunctuation::Comma => todo!(),
                            LxRosePunctuation::Period => {
                                break VdSynSentenceEnd::Period(pucntuation_token_idx)
                            }
                            LxRosePunctuation::Colon => todo!(),
                            LxRosePunctuation::Semicolon => todo!(),
                            LxRosePunctuation::Exclamation => todo!(),
                            LxRosePunctuation::Question => todo!(),
                            LxRosePunctuation::LeftCurl => todo!(),
                            LxRosePunctuation::RightCurl => todo!(),
                            LxRosePunctuation::LeftBox => todo!(),
                            LxRosePunctuation::RightBox => todo!(),
                            LxRosePunctuation::EscapedBackslash => todo!(),
                            LxRosePunctuation::EscapedLcurl => todo!(),
                            LxRosePunctuation::EscapedRcurl => todo!(),
                        }
                    }
                    LxRoseAstData::Math { .. } => todo!(),
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
            } else {
                break VdSynSentenceEnd::Void;
            }
        };
        let clauses = self.alloc_clauses(clauses);
        let data = VdSynSentenceData::Clauses { clauses, end };
        VdSynSentenceEntry { data }
    }
}
