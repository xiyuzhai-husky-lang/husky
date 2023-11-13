use super::*;
use husky_syn_decl::TypeAssociatedValSynDecl;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeAssociatedValHirDecl {
    pub path: TypeItemPath,
    pub hir_expr_region: HirEagerExprRegion,
}

impl TypeAssociatedValHirDecl {
    pub(super) fn from_syn(
        _path: TypeItemPath,
        syn_decl: TypeAssociatedValSynDecl,
        db: &dyn HirDeclDb,
    ) -> Self {
        let _builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        todo!()
        // let mut builder = HirEagerExprBuilder::new(db, syn_decl.expr_region(db));
        // let return_ty = builder.hir_ty(ethereal_signature_template.return_ty(db), db);
        // let hir_expr_region = builder.finish();
        // Self::new(db, path, return_ty, hir_expr_region)
    }
}
