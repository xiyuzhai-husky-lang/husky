use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar, constructor = pub(super) new)]
pub struct EnumTypeHirDefn {
    #[id]
    pub path: TypePath,
    pub hir_decl: EnumTypeHirDecl,
}
