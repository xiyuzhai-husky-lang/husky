use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar, constructor = pub(super) new)]
pub struct EnumTypeHirDefn {
    pub path: TypePath,
    pub hir_decl: EnumTypeHirDecl,
}
