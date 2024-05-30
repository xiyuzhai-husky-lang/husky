use husky_regional_token::RegionalTokenVerseIdx;
use idx_arena::*;

/// syntax tree down to TokenVerse level
#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CrateDeclAst {
    Err,
    /// let or return or require or a single `if` or `elif` or `else` or case branch
    BasicStmtOrBranch {
        /// the token group for the head
        regional_token_verse_idx: RegionalTokenVerseIdx,
        /// maybe have body or not
        body: Option<CrateDeclAstIdxRange>,
    },
    /// it's guaranteed that branches fall into `DefnAst::BasicStmtOrBranch`
    IfElseStmts {
        /// must have at least one `if` branch
        if_branch: CrateDeclAstIdx,
        /// may zero or multiple `elif` branches
        elif_branches: CrateDeclAstIdxRange,
        /// may have `else` branch or not
        else_branch: Option<CrateDeclAstIdx>,
    },
    /// it's guaranteed that branches fall into `DefnAst::BasicStmtOrBranch`
    MatchStmt {
        /// the token group for the head
        regional_token_verse_idx: RegionalTokenVerseIdx,
        /// ast idx for the head
        pattern_stmt: CrateDeclAstIdx,
        case_branches: CrateDeclAstIdxRange,
    },
}

pub type CrateDeclAstArena = Arena<CrateDeclAst>;
pub type CrateDeclAstArenaRef<'a> = ArenaRef<'a, CrateDeclAst>;
pub type CrateDeclAstIdx = ArenaIdx<CrateDeclAst>;
pub type CrateDeclAstIdxRange = ArenaIdxRange<CrateDeclAst>;
