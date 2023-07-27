use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar)]
pub struct ExternTypeHirDefn {
    pub path: TypePath,
    pub hir_decl: ExternTypeHirDecl,
}
