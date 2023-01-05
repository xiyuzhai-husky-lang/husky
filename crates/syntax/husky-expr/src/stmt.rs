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
        let_variable_pattern: ExprResult<LetVariablePattern>,
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
    Break {
        break_token: BreakToken,
    },
    Eval {
        expr: ExprIdx,
    },
    ForBetween {
        for_token: ForToken,
        condition: ExprResult<ExprIdx>,
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
    IfElse {},
    Match {},
}
