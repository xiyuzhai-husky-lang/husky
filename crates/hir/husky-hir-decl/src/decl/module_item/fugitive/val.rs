use super::*;
use husky_hir_expr::helpers::hir_expr_region;
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
        let syn_decl = path.syn_decl(db).expect("ok");
        Self::new(db, path, hir_expr_region(syn_decl.syn_expr_region(db), db))
    }
}
