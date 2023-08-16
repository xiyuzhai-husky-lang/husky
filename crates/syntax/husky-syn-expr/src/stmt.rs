mod branch_stmt;
mod error;
mod loop_stmt;

pub use self::branch_stmt::*;
pub use self::error::*;
pub use self::loop_stmt::*;

use crate::*;
use husky_token::*;
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange};

pub type SynStmtArena = Arena<SynStmt>;
pub type SynStmtIdx = ArenaIdx<SynStmt>;
pub type SynStmtIdxRange = ArenaIdxRange<SynStmt>;
pub type SynStmtMap<V> = ArenaMap<SynStmt, V>;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SynExprDb)]
pub enum SynStmt {
    Let {
        let_token: LetToken,
        let_variables_pattern: SynExprResult<LetVariableDecls>,
        assign_token: SynExprResult<EqToken>,
        initial_value: SynExprIdx,
    },
    Return {
        return_token: ReturnToken,
        result: SynExprIdx,
    },
    Require {
        require_token: RequireToken,
        condition: SynExprIdx,
    },
    Assert {
        assert_token: AssertToken,
        condition: SynExprIdx,
    },
    Break {
        break_token: BreakToken,
    },
    Eval {
        expr_idx: SynExprIdx,
        // todo: change this to EolOrEolSemicolonToken
        eol_semicolon: TokenResult<Option<EolSemicolonToken>>,
    },
    ForBetween {
        for_token: StmtForToken,
        particulars: SynForBetweenParticulars,
        frame_var_symbol_idx: CurrentSynSymbolIdx,
        eol_colon: SynExprResult<EolToken>,
        block: SynExprResult<SynStmtIdxRange>,
    },
    ForIn {
        for_token: StmtForToken,
        condition: SynExprResult<SynExprIdx>,
        eol_colon: SynExprResult<EolToken>,
        block: SynExprResult<SynStmtIdxRange>,
    },
    ForExt {
        forext_token: ForextToken,
        expr: SynExprIdx,
        eol_colon: SynExprResult<EolToken>,
        block: SynExprResult<SynStmtIdxRange>,
    },
    While {
        while_token: WhileToken,
        condition: SynExprResult<SynExprIdx>,
        eol_colon: SynExprResult<EolToken>,
        block: SynExprResult<SynStmtIdxRange>,
    },
    DoWhile {
        do_token: DoToken,
        while_token: WhileToken,
        condition: SynExprResult<SynExprIdx>,
        eol_colon: SynExprResult<EolToken>,
        block: SynExprResult<SynStmtIdxRange>,
    },
    IfElse {
        if_branch: SynIfBranch,
        elif_branches: Vec<SynElifBranch>,
        else_branch: Option<SynElseBranch>,
    },
    Match {
        match_token: MatchToken,
    },
    Err(StmtError),
}

impl From<StmtResult<SynStmt>> for SynStmt {
    fn from(value: StmtResult<SynStmt>) -> Self {
        match value {
            Ok(stmt) => stmt,
            Err(err) => SynStmt::Err(err),
        }
    }
}
