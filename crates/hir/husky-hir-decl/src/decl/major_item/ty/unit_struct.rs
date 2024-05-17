use husky_syn_decl::decl::UnitStructSynDecl;

use super::*;

#[salsa::interned]
pub struct UnitStructHirDecl {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

impl UnitStructHirDecl {
    pub(super) fn from_syn(path: TypePath, syn_decl: UnitStructSynDecl, db: &::salsa::Db) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let template_parameters =
            HirTemplateParameters::from_syn(syn_decl.template_parameters(db), &builder);
        Self::new(db, path, template_parameters, builder.finish().eager())
    }
}
