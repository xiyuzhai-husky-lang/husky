use super::*;
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef};
use latex_token::idx::LxMathTokenIdx;

pub enum VdSemClauseData {
    Verb,
    Let {
        let_token_idx: LxMathTokenIdx,
        left_dollar_token_idx: LxMathTokenIdx,
        // TODO: variable introduction
        right_dollar_token_idx: LxMathTokenIdx,
    },
    Assume {
        assume_token_idx: LxMathTokenIdx,
        right_dollar_token_idx: LxMathTokenIdx,
    },
    Then {
        then_token_idx: LxMathTokenIdx,
        right_dollar_token_idx: LxMathTokenIdx,
    },
}

pub type VdSemClauseArena = Arena<VdSemClauseData>;
pub type VdSemClauseArenaRef<'a> = ArenaRef<'a, VdSemClauseData>;
pub type VdSemClauseIdx = ArenaIdx<VdSemClauseData>;
pub type VdSemClauseIdxRange = ArenaIdxRange<VdSemClauseData>;
pub type VdSemClauseMap<T> = ArenaMap<VdSemClauseData, T>;
