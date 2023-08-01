use super::*;

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
        todo!()
        // let mut builder = HirEagerExprBuilder::new(db);
        // let hir_expr_region = builder.finish();
        // Self::new(db, path, hir_expr_region)
    }
}
