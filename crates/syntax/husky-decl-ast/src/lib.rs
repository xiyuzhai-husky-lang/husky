// base 1

use idx_arena::*;

/// syntax tree down to TokenVerse level
#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
