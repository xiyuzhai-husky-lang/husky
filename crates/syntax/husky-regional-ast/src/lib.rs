// base 1

use husky_ast::*;
use husky_regional_token::*;
use idx_arena::*;
use std::num::NonZeroU32;

/// syntax tree down to TokenGroup level
#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = AstDb)]
pub enum RegionalAst {
    Err,
    BasicStmtOrBranch {
        token_group_idx: RegionalTokenGroupIdx,
        body: Option<RegionalAstIdxRange>,
    },
    IfElseStmts {
        if_branch: RegionalAstIdx,
        elif_branches: RegionalAstIdxRange,
        else_branch: Option<RegionalAstIdx>,
    },
    MatchStmts {
        token_group_idx: RegionalTokenGroupIdx,
        pattern_stmt: RegionalAstIdx,
        case_stmts: RegionalAstIdxRange,
    },
}

pub type RegionalAstArena = Arena<RegionalAst>;
pub type RegionalAstArenaRef<'a> = ArenaRef<'a, RegionalAst>;
pub type RegionalAstIdx = ArenaIdx<RegionalAst>;
pub type RegionalAstIdxRange = ArenaIdxRange<RegionalAst>;
