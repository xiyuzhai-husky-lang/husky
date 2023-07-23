use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar)]
pub struct ExternTypeHirDefn {
    #[id]
    pub path: TypePath,
    pub hir_decl: ExternTypeHirDecl,
}
