use husky_expr::ExprRegion;

use crate::*;

pub trait RawSignatureDb: salsa::DbWithJar<RawSignatureJar> + DeclDb + RawTermDb {
    fn raw_signature_term_region(&self, expr_region: ExprRegion) -> &RawSignatureRawTermRegion;
    fn raw_signature(&self, decl: Decl) -> RawSignatureResultRef<RawSignature>;
    fn ty_raw_signature(&self, decl: TypeDecl) -> RawSignatureResultRef<TypeRawSignature>;
    fn trai_raw_signature(&self, decl: TraitDecl) -> RawSignatureResultRef<TraitRawSignature>;
    fn form_raw_signature(&self, decl: FormDecl) -> RawSignatureResultRef<FormRawSignature>;
}

impl<Db> RawSignatureDb for Db
where
    Db: salsa::DbWithJar<RawSignatureJar> + DeclDb + RawTermDb,
{
    fn raw_signature_term_region(&self, expr_region: ExprRegion) -> &RawSignatureRawTermRegion {
        raw_signature_term_region(self, expr_region)
    }

    fn raw_signature(&self, decl: Decl) -> RawSignatureResultRef<RawSignature> {
        raw_signature(self, decl)
    }

    fn ty_raw_signature(&self, decl: TypeDecl) -> RawSignatureResultRef<TypeRawSignature> {
        ty_raw_signature(self, decl)
    }

    fn trai_raw_signature(&self, decl: TraitDecl) -> RawSignatureResultRef<TraitRawSignature> {
        trai_raw_signature(self, decl).as_ref().copied()
    }

    fn form_raw_signature(&self, decl: FormDecl) -> RawSignatureResultRef<FormRawSignature> {
        form_raw_signature(self, decl)
    }
}
