use husky_expr::ExprRegion;

use crate::*;

pub trait SignatureDb: salsa::DbWithJar<SignatureJar> + DeclDb + RawTermDb {
    fn signature_term_region(&self, expr_region: ExprRegion) -> &SignatureTermRegion;
    fn signature(&self, decl: Decl) -> SignatureResult<Signature>;
    fn ty_signature(&self, decl: TypeDecl) -> SignatureResult<TypeSignature>;
    fn trai_signature(&self, decl: TraitDecl) -> SignatureResult<TraitSignature>;
    fn form_signature(&self, decl: FormDecl) -> SignatureResult<FormSignature>;
}

impl<Db> SignatureDb for Db
where
    Db: salsa::DbWithJar<SignatureJar> + DeclDb + RawTermDb,
{
    fn signature_term_region(&self, expr_region: ExprRegion) -> &SignatureTermRegion {
        signature_term_region(self, expr_region)
    }

    fn signature(&self, decl: Decl) -> SignatureResult<Signature> {
        signature(self, decl)
    }

    fn ty_signature(&self, decl: TypeDecl) -> SignatureResult<TypeSignature> {
        ty_signature(self, decl)
    }

    fn trai_signature(&self, decl: TraitDecl) -> SignatureResult<TraitSignature> {
        trai_signature(self, decl)
    }

    fn form_signature(&self, decl: FormDecl) -> SignatureResult<FormSignature> {
        form_signature(self, decl)
    }
}
