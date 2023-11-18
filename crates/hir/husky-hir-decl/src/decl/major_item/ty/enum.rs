use super::*;
use husky_syn_decl::EnumTypeSynDecl;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar, constructor = new)]
pub struct EnumTypeHirDecl {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

impl EnumTypeHirDecl {
    pub(super) fn from_syn(path: TypePath, syn_decl: EnumTypeSynDecl, db: &dyn HirDeclDb) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let template_parameters =
            HirTemplateParameters::from_syn(syn_decl.template_parameters(db), &builder);
        Self::new(
            db,
            path,
            template_parameters,
            hir_eager_expr_region(syn_decl.syn_expr_region(db), db),
        )
    }
}
