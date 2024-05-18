use super::*;
use husky_syn_decl::decl::TypeAssocValSynDecl;

#[salsa::interned]
pub struct TypeAssocValHirDecl {
    pub path: TypeItemPath,
    pub return_ty: HirType,
    pub hir_expr_region: HirExprRegion,
}

impl TypeAssocValHirDecl {
    pub(super) fn from_syn(
        _path: TypeItemPath,
        syn_decl: TypeAssocValSynDecl,
        db: &::salsa::Db,
    ) -> Self {
        let _builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        todo!()
        // let mut builder = HirEagerExprBuilder::new(db, syn_decl.expr_region(db));
        // let return_ty = builder.hir_ty(eth_template.return_ty(db), db);
        // let hir_expr_region = builder.finish();
        // Self::new(db, path, return_ty, hir_expr_region)
    }
}
