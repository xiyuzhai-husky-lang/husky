use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar)]
pub struct EnumPropsVariantHirDefn {
    #[id]
    pub path: TypeVariantPath,
    pub decl: EnumPropsTypeVariantHirDecl,
}
