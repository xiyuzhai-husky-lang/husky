use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeMemoizedFieldHirDecl {
    pub impl_block: TypeImplBlockHirDecl,
    pub return_ty: EtherealTerm,
}
