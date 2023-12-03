use super::*;
use husky_syn_decl::TypeAliasSynDecl;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeAliasHirDecl {
    pub path: FugitivePath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub ty: HirType,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

impl TypeAliasHirDecl {
    pub(super) fn from_syn(
        path: FugitivePath,
        syn_decl: TypeAliasSynDecl,
        db: &::salsa::Db,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let template_parameters =
            HirTemplateParameters::from_syn(syn_decl.template_parameters(db), &builder);
        let ty = builder.hir_ty(todo!()).unwrap();
        Self::new(db, path, template_parameters, ty, builder.finish().eager())
    }
}
