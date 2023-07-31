use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeImplBlockHirDecl {
    pub path: TypeImplBlockPath,
    #[return_ref]
    pub template_parameters: EtherealTemplateParameters,
    pub self_ty: HirType,
    pub hir_expr_region: HirEagerExprRegion,
}
