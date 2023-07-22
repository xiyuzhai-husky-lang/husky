use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct EnumPropsTypeVariantHirDecl {
    pub parent_ty_template: EnumHirDecl,
}
