use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct EnumTupleTypeVariantHirDecl {
    pub parent_ty_template: EnumTypeHirDecl,
}
