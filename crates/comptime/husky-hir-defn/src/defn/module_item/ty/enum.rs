use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar, constructor = pub(super) new)]
pub struct EnumTypeHirDefn {
    #[id]
    pub path: TypePath,
    pub decl: EnumTypeHirDecl,
}
