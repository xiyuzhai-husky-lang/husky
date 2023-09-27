mod if_else_stmt;
mod loop_stmt;
mod match_stmt;

pub use self::if_else_stmt::*;
pub use self::loop_stmt::*;
pub use self::match_stmt::*;

use husky_regional_token::{
    AssertRegionalToken, BreakRegionalToken, DoRegionalToken, EolRegionalToken,
    EolSemicolonRegionalToken, EolWithRegionalToken, ForextRegionalToken, LetRegionalToken,
    MatchRegionalToken, RegionalEqToken, RequireRegionalToken, ReturnRegionalToken,
    StmtForRegionalToken, WhileRegionalToken,
};
use husky_token_data::TokenDataResult;
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange};

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub enum SemaStmtData {
    Let {
        let_token: LetRegionalToken,
        let_variables_pattern: LetPatternObelisk,
        assign_token: RegionalEqToken,
        initial_value: SemaExprIdx,
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
        expr_idx: SemaExprIdx,
        // todo: change this to EolOrEolSemicolonToken
        eol_semicolon: TokenDataResult<Option<EolSemicolonRegionalToken>>,
    },
    ForBetween {
        for_token: StmtForRegionalToken,
        particulars: SemaForBetweenParticulars,
        frame_var_symbol_idx: SynCurrentSymbolIdx,
        eol_colon: EolRegionalToken,
        block: SemaStmtIdxRange,
    },
    ForIn {
        for_token: StmtForRegionalToken,
        condition: SemaExprIdx,
        eol_colon: EolRegionalToken,
        block: SemaStmtIdxRange,
    },
    ForExt {
        forext_token: ForextRegionalToken,
        particulars: SemaForextParticulars,
        eol_colon: SemaExprTypeResult<EolRegionalToken>,
        block: SemaStmtIdxRange,
    },
    While {
        while_token: WhileRegionalToken,
        condition: SemaExprTypeResult<SemaExprIdx>,
        eol_colon: SemaExprTypeResult<EolRegionalToken>,
        block: SemaStmtIdxRange,
    },
    DoWhile {
        do_token: DoRegionalToken,
        while_token: WhileRegionalToken,
        condition: SemaExprTypeResult<SemaExprIdx>,
        eol_colon: SemaExprTypeResult<EolRegionalToken>,
        block: SemaStmtIdxRange,
    },
    IfElse {
        if_branch: SemaIfBranch,
        elif_branches: Vec<SemaElifBranch>,
        else_branch: Option<SemaElseBranch>,
    },
    Match {
        match_token: MatchRegionalToken,
        match_expr: SemaExprTypeResult<SemaExprIdx>,
        eol_with_token: SemaExprTypeResult<EolWithRegionalToken>,
        case_branches: Vec<SemaCaseBranch>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct SemaStmtEntry {
    data_result: SemaExprDataResult<SemaStmtData>,
    ty_result: SemaExprTypeResult<FluffyTerm>,
}

impl SemaStmtEntry {
    pub fn new(
        data_result: SemaExprDataResult<SemaStmtData>,
        ty_result: SemaExprTypeResult<FluffyTerm>,
    ) -> Self {
        Self {
            data_result,
            ty_result,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SemaStmtArena(Arena<SemaStmtEntry>);

impl SemaStmtArena {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemaStmtIdx(ArenaIdx<SemaStmtEntry>);

pub type SemaStmtIdxRange = ArenaIdxRange<SemaStmtEntry>;
pub type SemaStmtMap<V> = ArenaMap<SemaStmtEntry, V>;
