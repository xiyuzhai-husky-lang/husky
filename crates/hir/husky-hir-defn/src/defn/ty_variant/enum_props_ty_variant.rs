use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar)]
pub struct EnumPropsVariantHirDefn {
    pub path: TypeVariantPath,
    pub hir_decl: EnumPropsTypeVariantHirDecl,
}
