use husky_expr::ExprRegion;

use crate::*;

pub trait SignatureDb: salsa::DbWithJar<SignatureJar> + DeclDb + RawTermDb {
    fn signature_term_region(&self, expr_region: ExprRegion) -> &SignatureTermRegion;
    fn signature(&self, decl: Decl) -> SignatureResultRef<Signature>;
    fn ty_signature(&self, decl: TypeDecl) -> SignatureResultRef<TypeSignature>;
    fn trai_signature(&self, decl: TraitDecl) -> SignatureResultRef<TraitSignature>;
    fn form_signature(&self, decl: FormDecl) -> SignatureResultRef<FormSignature>;
}

impl<Db> SignatureDb for Db
where
    Db: salsa::DbWithJar<SignatureJar> + DeclDb + RawTermDb,
{
    fn signature_term_region(&self, expr_region: ExprRegion) -> &SignatureTermRegion {
        signature_term_region(self, expr_region)
    }

    fn signature(&self, decl: Decl) -> SignatureResultRef<Signature> {
        signature(self, decl)
    }

    fn ty_signature(&self, decl: TypeDecl) -> SignatureResultRef<TypeSignature> {
        ty_signature(self, decl)
    }

    fn trai_signature(&self, decl: TraitDecl) -> SignatureResultRef<TraitSignature> {
        trai_signature(self, decl).as_ref().copied()
    }

    fn form_signature(&self, decl: FormDecl) -> SignatureResultRef<FormSignature> {
        form_signature(self, decl)
    }
}
