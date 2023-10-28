mod if_else_stmt;
mod loop_stmt;
mod match_stmt;

pub use self::if_else_stmt::*;
pub use self::loop_stmt::*;
pub use self::match_stmt::*;

use husky_regional_token::{
    AssertRegionalToken, BreakRegionalToken, DoRegionalToken, EolRegionalToken,
    EolSemicolonRegionalToken, EolWithRegionalToken, EqRegionalToken, ForextRegionalToken,
    LetRegionalToken, MatchRegionalToken, RequireRegionalToken, ReturnRegionalToken,
    StmtForRegionalToken, WhileRegionalToken,
};
use husky_token_data::TokenDataResult;
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange, ArenaRef};

use crate::{obelisks::let_pattern_obelisk::LetPatternSemaObelisk, *};

#[derive(Debug, PartialEq, Eq)]
pub enum SemaStmtData {
    Let {
        let_token: LetRegionalToken,
        let_pattern_sema_obelisk: LetPatternSemaObelisk,
        eq_token: EqRegionalToken,
        initial_value_sema_expr_idx: SemaExprIdx,
    },
    Return {
        return_token: ReturnRegionalToken,
        result: SemaExprIdx,
    },
    Require {
        require_token: RequireRegionalToken,
        condition: SemaExprIdx,
    },
    Assert {
        assert_token: AssertRegionalToken,
        condition: SemaExprIdx,
    },
    Break {
        break_token: BreakRegionalToken,
    },
    Eval {
        sema_expr_idx: SemaExprIdx,
        // todo: change this to EolOrEolSemicolonToken
        eol_semicolon: TokenDataResult<Option<EolSemicolonRegionalToken>>,
    },
    ForBetween {
        for_token: StmtForRegionalToken,
        particulars: SemaForBetweenParticulars,
        for_loop_var_symbol_idx: SynCurrentSymbolIdx,
        eol_colon: EolRegionalToken,
        block: SemaStmtIdxRange,
    },
    ForIn {
        for_token: StmtForRegionalToken,
        condition: SemaExprIdx,
        eol_colon: EolRegionalToken,
        block: SemaStmtIdxRange,
    },
    Forext {
        forext_token: ForextRegionalToken,
        particulars: SemaForextParticulars,
        eol_colon: EolRegionalToken,
        block: SemaStmtIdxRange,
    },
    While {
        while_token: WhileRegionalToken,
        condition: SemaExprIdx,
        eol_colon: EolRegionalToken,
        block: SemaStmtIdxRange,
    },
    DoWhile {
        do_token: DoRegionalToken,
        while_token: WhileRegionalToken,
        condition: SemaExprIdx,
        eol_colon: EolRegionalToken,
        block: SemaStmtIdxRange,
    },
    IfElse {
        sema_if_branch: SemaIfBranch,
        sema_elif_branches: Vec<SemaElifBranch>,
        sema_else_branch: Option<SemaElseBranch>,
    },
    Match {
        match_token: MatchRegionalToken,
        match_target_sema_expr_idx: SemaExprIdx,
        eol_with_token: EolWithRegionalToken,
        sema_case_branches: Vec<SemaCaseBranch>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct SemaStmtEntry {
    data_result: SemaExprDataResult<SemaStmtData>,
    ty_result: SemaExprTypeResult<FluffyTerm>,
}

#[derive(Debug, Default)]
pub(crate) struct SemaStmtBatch {
    entries: SmallVec<[SemaStmtEntry; 8]>,
}

impl SemaStmtBatch {
    pub(crate) fn add(
        &mut self,
        (data_result, ty_result): (
            SemaExprDataResult<SemaStmtData>,
            SemaExprTypeResult<FluffyTerm>,
        ),
    ) {
        self.entries.push(SemaStmtEntry {
            data_result,
            ty_result,
        })
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct SemaStmtArena(Arena<SemaStmtEntry>);

impl SemaStmtArena {
    pub(crate) fn alloc_batch(&mut self, batch: SemaStmtBatch) -> SemaStmtIdxRange {
        SemaStmtIdxRange(self.0.alloc_batch(batch.entries))
    }

    pub fn arena_ref<'a>(&'a self) -> SemaStmtArenaRef<'a> {
        SemaStmtArenaRef(self.0.arena_ref())
    }
}

pub struct SemaStmtArenaRef<'a>(ArenaRef<'a, SemaStmtEntry>);

impl<'a> std::ops::Index<SemaStmtIdx> for SemaStmtArenaRef<'a> {
    type Output = SemaStmtEntry;

    fn index(&self, index: SemaStmtIdx) -> &Self::Output {
        &self.0[index.0]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemaStmtIdx(ArenaIdx<SemaStmtEntry>);

impl SemaStmtIdx {
    pub fn data<'a>(self, arena_ref: SemaStmtArenaRef<'a>) -> &'a SemaStmtData {
        arena_ref
            .0
            .index(self.0)
            .data_result
            .as_ref()
            .expect("no error")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemaStmtIdxRange(ArenaIdxRange<SemaStmtEntry>);

impl SemaStmtIdxRange {
    pub fn iter(&self) -> impl Iterator<Item = SemaStmtIdx> {
        self.0.into_iter().map(SemaStmtIdx)
    }
}

impl IntoIterator for SemaStmtIdxRange {
    type Item = SemaStmtIdx;

    type IntoIter = std::iter::Map<
        <ArenaIdxRange<stmt::SemaStmtEntry> as IntoIterator>::IntoIter,
        fn(ArenaIdx<SemaStmtEntry>) -> SemaStmtIdx,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().map(SemaStmtIdx)
    }
}

impl IntoIterator for &SemaStmtIdxRange {
    type Item = SemaStmtIdx;

    type IntoIter = std::iter::Map<
        <ArenaIdxRange<stmt::SemaStmtEntry> as IntoIterator>::IntoIter,
        fn(ArenaIdx<SemaStmtEntry>) -> SemaStmtIdx,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().map(SemaStmtIdx)
    }
}

pub type SemaStmtMap<V> = ArenaMap<SemaStmtEntry, V>;
