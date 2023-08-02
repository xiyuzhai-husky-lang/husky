use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeAssociatedTypeHirDecl {
    pub path: TypeItemPath,
}

impl TypeAssociatedTypeHirDecl {
    pub(super) fn from_ethereal(
        path: TypeItemPath,
        ethereal_signature_template: TypeAssociatedTypeEtherealSignatureTemplate,
        db: &dyn HirDeclDb,
    ) -> Self {
        // let mut builder = HirEagerExprBuilder::new(db);
        // // let return_ty = HirType::from_ethereal(ethereal_signature_template.return_ty(db), db);
        // let hir_expr_region = builder.finish();
        Self::new(db, path)
    }
}
