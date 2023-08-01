use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct UnitStructTypeHirDecl {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub hir_expr_region: HirEagerExprRegion,
}

impl UnitStructTypeHirDecl {
    pub(super) fn from_ethereal(
        path: TypePath,
        ethereal_signature_template: UnitStructTypeEtherealSignatureTemplate,
        db: &dyn HirDeclDb,
    ) -> Self {
        todo!()
    }
}
