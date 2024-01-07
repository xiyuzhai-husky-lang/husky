// base 1



use idx_arena::*;

/// syntax tree down to TokenGroup level
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::debug_with_db(db = AstDb, jar = AstJar)]
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
