use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar)]
pub struct UnionTypeHirDefn {
    pub path: TypePath,
    pub hir_decl: UnionTypeHirDecl,
}
