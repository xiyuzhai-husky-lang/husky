use super::*;
use husky_syn_decl::RecordTypeSynDecl;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct RecordTypeHirDecl {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

impl RecordTypeHirDecl {
    pub(super) fn from_syn(_path: TypePath, syn_decl: RecordTypeSynDecl, db: &::salsa::Db) -> Self {
        let _builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        todo!()
    }
}
