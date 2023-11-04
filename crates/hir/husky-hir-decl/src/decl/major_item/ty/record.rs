use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct RecordTypeHirDecl {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub hir_expr_region: HirEagerExprRegion,
}

impl RecordTypeHirDecl {
    pub(super) fn from_syn(
        path: TypePath,
        ethereal_signature_template: RecordTypeEtherealSignatureTemplate,
        db: &dyn HirDeclDb,
    ) -> Self {
        todo!()
    }
}
