use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct ValHirDecl {
    pub path: FugitivePath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub hir_expr_region: HirExprRegion,
}
