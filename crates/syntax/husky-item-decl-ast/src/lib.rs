// base 1

use idx_arena::*;

/// syntax tree down to TokenVerse level
#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ItemDeclAst {
    Identifiable {},
    ImplBlock,
    TypeVariant,
    Attr,
    Submodule,
}

pub type ItemDeclAstArena = Arena<ItemDeclAst>;
pub type ItemDeclAstArenaRef<'a> = ArenaRef<'a, ItemDeclAst>;
pub type ItemDeclAstIdx = ArenaIdx<ItemDeclAst>;
pub type ItemDeclAstIdxRange = ArenaIdxRange<ItemDeclAst>;
