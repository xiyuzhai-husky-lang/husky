use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct GnHirDecl {
    pub path: FugitivePath,
    #[return_ref]
    pub template_parameters: EtherealTemplateParameters,
    pub hir_expr_region: HirEagerExprRegion,
}
