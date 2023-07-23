use super::*;

#[salsa::tracked(db = HirDefnDb, jar = HirDefnJar)]
pub struct EnumTupleVariantHirDefn {
    #[id]
    pub path: TypeVariantPath,
    pub hir_decl: EnumTupleTypeVariantHirDecl,
}
