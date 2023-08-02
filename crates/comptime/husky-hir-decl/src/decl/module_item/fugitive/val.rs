use super::*;
use husky_syn_decl::HasSynDecl;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct ValFugitiveHirDecl {
    pub path: FugitivePath,
    pub hir_expr_region: HirExprRegion,
}

impl ValFugitiveHirDecl {
    pub(super) fn from_ethereal(
        path: FugitivePath,
        ethereal_signature_template: ValFugitiveEtherealSignatureTemplate,
        db: &dyn HirDeclDb,
    ) -> Self {
        let decl = path.syn_decl(db).expect("ok");
        let mut builder = HirExprBuilder::new(db, decl.syn_expr_region(db));
        let hir_expr_region = builder.finish();
        Self::new(db, path, hir_expr_region)
    }
}
