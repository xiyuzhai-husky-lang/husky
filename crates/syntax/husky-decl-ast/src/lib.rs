// base 1

use husky_ast::*;
use husky_regional_token::*;
use idx_arena::*;
use std::num::NonZeroU32;

/// syntax tree down to TokenGroup level
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = AstDb)]
pub enum DeclAst {
    Identifiable {},
    ImplBlock,
    TypeVariant,
    Attr,
    Submodule,
}

pub type DeclAstArena = Arena<DeclAst>;
pub type DeclAstArenaRef<'a> = ArenaRef<'a, DeclAst>;
pub type DeclAstIdx = ArenaIdx<DeclAst>;
pub type DeclAstIdxRange = ArenaIdxRange<DeclAst>;
