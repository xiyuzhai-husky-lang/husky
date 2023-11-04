use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar, constructor = new)]
pub struct EnumTypeHirDecl {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub hir_expr_region: HirEagerExprRegion,
}

impl EnumTypeHirDecl {
    pub(super) fn from_syn(
        path: TypePath,
        ethereal_signature_template: EnumTypeEtherealSignatureTemplate,
        db: &dyn HirDeclDb,
    ) -> Self {
        let TypeSynDecl::Enum(syn_decl) = path.syn_decl(db).expect("hir stage ok") else {
            unreachable!()
        };
        let template_parameters = HirTemplateParameters::from_syn(
            ethereal_signature_template.template_parameters(db),
            db,
        );
        Self::new(
            db,
            path,
            template_parameters,
            hir_eager_expr_region(syn_decl.syn_expr_region(db), db),
        )
    }
}
