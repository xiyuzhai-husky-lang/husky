use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeImplBlockHirDecl {
    pub path: TypeImplBlockPath,
    #[return_ref]
    pub generic_parameters: EtherealGenericParameters,
    pub self_ty: EtherealTerm,
    pub hir_expr_region: HirEagerExprRegion,
}
