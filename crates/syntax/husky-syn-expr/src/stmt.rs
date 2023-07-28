mod error;
mod loop_stmt;

pub use error::*;
pub use loop_stmt::*;

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
        let_variable_pattern: SynExprResult<LetVariableDecls>,
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
    },
    ForBetween {
        for_token: StmtForToken,
        particulars: ForBetweenParticulars,
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
        if_branch: IfBranch,
        elif_branches: Vec<ElifBranch>,
        else_branch: Option<ElseBranch>,
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

#[derive(Debug, PartialEq, Eq)]
pub struct IfBranch {
    pub if_token: IfToken,
    pub condition: SynExprResult<SynExprIdx>,
    pub eol_colon: SynExprResult<EolToken>,
    pub block: SynExprResult<SynStmtIdxRange>,
}

impl IfBranch {
    pub fn condition(&self) -> Result<&SynExprIdx, &ExprError> {
        self.condition.as_ref()
    }

    pub fn eol_colon_token(&self) -> Result<&EolToken, &ExprError> {
        self.eol_colon.as_ref()
    }

    pub fn block(&self) -> Result<SynStmtIdxRange, &ExprError> {
        self.block.as_ref().copied()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ElifBranch {
    pub elif_token: ElifToken,
    pub condition: SynExprResult<SynExprIdx>,
    pub eol_colon: SynExprResult<EolToken>,
    pub block: SynExprResult<SynStmtIdxRange>,
}

impl ElifBranch {
    pub fn condition(&self) -> Result<&SynExprIdx, &ExprError> {
        self.condition.as_ref()
    }

    pub fn eol_colon(&self) -> Result<&EolToken, &ExprError> {
        self.eol_colon.as_ref()
    }

    pub fn block(&self) -> Result<SynStmtIdxRange, &ExprError> {
        self.block.as_ref().copied()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ElseBranch {
    pub else_token: ElseToken,
    pub eol_colon: SynExprResult<EolToken>,
    pub block: SynExprResult<SynStmtIdxRange>,
}

impl ElseBranch {
    pub fn block(&self) -> Result<SynStmtIdxRange, &ExprError> {
        self.block.as_ref().copied()
    }
}
