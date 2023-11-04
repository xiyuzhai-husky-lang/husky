use super::*;
use husky_syn_decl::UnionTypeSynDecl;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct UnionTypeHirDecl {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub hir_expr_region: HirEagerExprRegion,
}

impl UnionTypeHirDecl {
    pub(super) fn from_syn(path: TypePath, syn_decl: UnionTypeSynDecl, db: &dyn HirDeclDb) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        todo!()
    }
}
