use super::*;

#[salsa::interned(db = HirDefnDb, jar = HirDefnJar)]
pub struct EnumTupleVariantHirDefn {
    pub path: TypeVariantPath,
    pub hir_decl: EnumTupleTypeVariantHirDecl,
}
