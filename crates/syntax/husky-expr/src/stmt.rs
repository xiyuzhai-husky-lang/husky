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
    },
    Return {
        return_token: ReturnToken,
    },
    Require {
        require_token: RequireToken,
    },
    Break {
        break_token: BreakToken,
    },
    Eval {},
    IfElse {},
    Match {},
}
