use super::*;

#[salsa::tracked(jar = SignatureJar,return_ref)]
pub fn alien_ty_signature(
    db: &dyn SignatureDb,
    decl: AlienTypeDecl,
) -> SignatureOutcome<AlienTypeSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    Success(AlienTypeSignature::new(
        db,
        ImplicitParameterSignatures::from_decl(
            decl.implicit_parameters(db),
            &signature_term_region,
            term_menu,
        ),
    ))
}

#[salsa::interned(jar = SignatureJar)]
pub struct AlienTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
}

impl AlienTypeSignature {}
