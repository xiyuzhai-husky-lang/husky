use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeAssociatedFnHirDecl {
    pub path: TypeItemPath,
    pub self_ty: EtherealTerm,
    pub generic_parameters: EtherealGenericParameters,
    pub parenic_parameters: ParenicEtherealParameters,
    pub return_ty: EtherealTerm,
    pub ty: EtherealTerm,
    pub hir_expr_region: HirEagerExprRegion,
}
