use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar)]
pub struct EnumUnitVariantHirDefn {
    #[id]
    pub path: TypeVariantPath,
    pub decl: EnumUnitTypeVariantHirDecl,
}
