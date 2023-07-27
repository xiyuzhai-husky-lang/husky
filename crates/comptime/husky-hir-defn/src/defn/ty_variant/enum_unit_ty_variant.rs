use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar)]
pub struct EnumUnitVariantHirDefn {
    pub path: TypeVariantPath,
    pub hir_decl: EnumUnitTypeVariantHirDecl,
}
