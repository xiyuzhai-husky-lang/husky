use crate::*;

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct GnSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
}

impl HasSignature for GnDecl {
    type Signature = GnSignature;

    fn signature(self, db: &dyn SignatureDb) -> SignatureResult<Self::Signature> {
        gn_signature(db, self)
    }
}

#[salsa::tracked(jar = SignatureJar)]
pub fn gn_signature(db: &dyn SignatureDb, decl: GnDecl) -> SignatureResult<GnSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    let implicit_parameters = ImplicitParameterSignatures::from_decl(
        decl.implicit_parameters(db),
        &signature_term_region,
        raw_term_menu,
    );
    Ok(GnSignature::new(db, implicit_parameters))
}

impl GnSignature {}
