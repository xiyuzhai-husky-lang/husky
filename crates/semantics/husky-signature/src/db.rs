use husky_decr::DecrDb;
use husky_expr::ExprRegion;

use crate::*;

pub trait SignatureDb: salsa::DbWithJar<SignatureJar> + DecrDb + DeclarativeTermDb {
    fn signature_term_region(&self, expr_region: ExprRegion) -> &SignatureRegion;
    fn ty_signature_from_decl(&self, decl: TypeDecl) -> SignatureResult<TypeSignature>;
    fn trai_signature(&self, decl: TraitDecl) -> SignatureResult<TraitSignature>;
    fn ty_item_signature(&self, path: TypeItemPath) -> SignatureResult<TypeItemSignature>;
}

impl<Db> SignatureDb for Db
where
    Db: salsa::DbWithJar<SignatureJar> + DecrDb + DeclarativeTermDb,
{
    fn signature_term_region(&self, expr_region: ExprRegion) -> &SignatureRegion {
        signature_term_region(self, expr_region)
    }

    fn ty_signature_from_decl(&self, decl: TypeDecl) -> SignatureResult<TypeSignature> {
        ty_signature_from_decl(self, decl)
    }

    fn trai_signature(&self, decl: TraitDecl) -> SignatureResult<TraitSignature> {
        trai_signature_from_decl(self, decl)
    }

    fn ty_item_signature(&self, path: TypeItemPath) -> SignatureResult<TypeItemSignature> {
        ty_item_signature(self, path)
    }
}
