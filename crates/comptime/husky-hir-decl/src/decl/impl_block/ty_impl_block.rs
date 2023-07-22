use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeImplBlockHirDecl {
    #[return_ref]
    pub generic_parameters: EtherealGenericParameters,
    pub self_ty: EtherealTerm,
}
