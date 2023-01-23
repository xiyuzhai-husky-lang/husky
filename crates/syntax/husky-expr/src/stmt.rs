mod error;
mod loop_stmt;

pub use error::*;
pub use loop_stmt::*;

use crate::*;
use husky_token::*;
use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange};

pub type StmtArena = Arena<Stmt>;
pub type StmtIdx = ArenaIdx<Stmt>;
pub type StmtIdxRange = ArenaIdxRange<Stmt>;
pub type StmtMap<V> = ArenaMap<Stmt, V>;

#[derive(Debug, PartialEq, Eq)]
pub enum Stmt {
    Let {
        let_token: LetToken,
        let_variable_pattern: ExprResult<LetVariablesPattern>,
        assign_token: ExprResult<AssignToken>,
        initial_value: ExprResult<ExprIdx>,
    },
    Return {
        return_token: ReturnToken,
        result: ExprResult<ExprIdx>,
    },
    Require {
        require_token: RequireToken,
        condition: ExprResult<ExprIdx>,
    },
    Assert {
        assert_token: AssertToken,
        condition: ExprResult<ExprIdx>,
    },
    Break {
        break_token: BreakToken,
    },
    Eval {
        expr: ExprIdx,
    },
    ForBetween {
        for_token: ForToken,
        particulars: ForBetweenParticulars,
        eol_colon: ExprResult<EolColonToken>,
        block: ExprResult<StmtIdxRange>,
    },
    ForIn {
        for_token: ForToken,
        condition: ExprResult<ExprIdx>,
        eol_colon: ExprResult<EolColonToken>,
        block: ExprResult<StmtIdxRange>,
    },
    ForExt {
        forext_token: ForextToken,
        eol_colon: ExprResult<EolColonToken>,
        block: ExprResult<StmtIdxRange>,
    },
    While {
        while_token: WhileToken,
        condition: ExprResult<ExprIdx>,
        eol_colon: ExprResult<EolColonToken>,
        block: ExprResult<StmtIdxRange>,
    },
    DoWhile {
        do_token: DoToken,
        while_token: WhileToken,
        condition: ExprResult<ExprIdx>,
        eol_colon: ExprResult<EolColonToken>,
        block: ExprResult<StmtIdxRange>,
    },
    IfElse {
        if_branch: IfBranch,
        elif_branches: Vec<ElifBranch>,
        else_branch: Option<ElseBranch>,
    },
    Match {},
    Err(StmtError),
}

impl From<StmtResult<Stmt>> for Stmt {
    fn from(value: StmtResult<Stmt>) -> Self {
        match value {
            Ok(stmt) => stmt,
            Err(err) => Stmt::Err(err),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct IfBranch {
    pub if_token: IfToken,
    pub condition: ExprResult<ExprIdx>,
    pub eol_colon: ExprResult<EolColonToken>,
    pub block: ExprResult<StmtIdxRange>,
}

impl IfBranch {
    pub fn condition(&self) -> Result<&ArenaIdx<Expr>, &ExprError> {
        self.condition.as_ref()
    }

    pub fn eol_colon(&self) -> Result<&EolColonToken, &ExprError> {
        self.eol_colon.as_ref()
    }

    pub fn block(&self) -> Result<&ArenaIdxRange<Stmt>, &ExprError> {
        self.block.as_ref()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ElifBranch {
    pub elif_token: ElifToken,
    pub condition: ExprResult<ExprIdx>,
    pub eol_colon: ExprResult<EolColonToken>,
    pub block: ExprResult<StmtIdxRange>,
}

impl ElifBranch {
    pub fn condition(&self) -> Result<&ArenaIdx<Expr>, &ExprError> {
        self.condition.as_ref()
    }

    pub fn eol_colon(&self) -> Result<&EolColonToken, &ExprError> {
        self.eol_colon.as_ref()
    }

    pub fn block(&self) -> Result<&ArenaIdxRange<Stmt>, &ExprError> {
        self.block.as_ref()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ElseBranch {
    pub else_token: ElseToken,
    pub eol_colon: ExprResult<EolColonToken>,
    pub block: ExprResult<StmtIdxRange>,
}
