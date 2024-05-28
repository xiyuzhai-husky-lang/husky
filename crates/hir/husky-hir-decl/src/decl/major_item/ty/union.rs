use super::*;
use husky_syn_decl::decl::major_item::ty::union::UnionSynDecl;

#[salsa::interned]
pub struct UnionHirDecl {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

impl UnionHirDecl {
    pub(super) fn from_syn(_path: TypePath, syn_decl: UnionSynDecl, db: &::salsa::Db) -> Self {
        let _builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        todo!()
    }
}
