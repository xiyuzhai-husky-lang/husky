// base 1

use husky_ast::*;
use husky_regional_token::RegionalTokenGroupIdx;
use husky_regional_token::*;
use idx_arena::*;
use std::num::NonZeroU32;

/// syntax tree down to TokenGroup level
#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = AstDb)]
pub enum DefnAst {
    Err,
    BasicStmtOrBranch {
        regional_token_group_idx: RegionalTokenGroupIdx,
        body: Option<DefnAstIdxRange>,
    },
    IfElseStmts {
        if_branch: DefnAstIdx,
        elif_branches: DefnAstIdxRange,
        else_branch: Option<DefnAstIdx>,
    },
    MatchStmt {
        regional_token_group_idx: RegionalTokenGroupIdx,
        pattern_stmt: DefnAstIdx,
        case_branches: DefnAstIdxRange,
    },
}

pub type DefnAstArena = Arena<DefnAst>;
pub type DefnAstArenaRef<'a> = ArenaRef<'a, DefnAst>;
pub type DefnAstIdx = ArenaIdx<DefnAst>;
pub type DefnAstIdxRange = ArenaIdxRange<DefnAst>;
