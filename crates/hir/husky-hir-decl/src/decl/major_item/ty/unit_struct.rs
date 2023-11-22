use husky_syn_decl::UnitStructTypeSynDecl;

use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct UnitStructHirDecl {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

impl UnitStructHirDecl {
    pub(super) fn from_syn(
        _path: TypePath,
        syn_decl: UnitStructTypeSynDecl,
        db: &dyn HirDeclDb,
    ) -> Self {
        let _builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        todo!()
    }
}
