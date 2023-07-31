use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeAssociatedFnHirDecl {
    pub path: TypeItemPath,
    pub self_ty: HirType,
    pub template_parameters: EtherealTemplateParameters,
    pub parenic_parameters: ParenicEtherealParameters,
    pub return_ty: HirType,
    pub ty: HirType,
    pub hir_expr_region: HirEagerExprRegion,
}
