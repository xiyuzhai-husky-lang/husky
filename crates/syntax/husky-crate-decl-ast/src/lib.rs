use idx_arena::*;

/// syntax tree down to TokenVerse level
#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CrateDeclAst {
    Narrative,
}

pub type CrateDeclAstArena = Arena<CrateDeclAst>;
pub type CrateDeclAstArenaRef<'a> = ArenaRef<'a, CrateDeclAst>;
pub type CrateDeclAstIdx = ArenaIdx<CrateDeclAst>;
pub type CrateDeclAstIdxRange = ArenaIdxRange<CrateDeclAst>;
