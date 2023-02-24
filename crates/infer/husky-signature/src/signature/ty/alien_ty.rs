use super::*;

#[salsa::tracked(jar = SignatureJar,return_ref)]
pub fn alien_ty_signature(
    db: &dyn SignatureDb,
    decl: AlienTypeDecl,
) -> SignatureResult<AlienTypeSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    Ok(AlienTypeSignature::new(
        db,
        ImplicitParameterSignatures::from_decl(
            decl.implicit_parameters(db)?,
            &signature_term_region,
            term_menu,
        ),
    ))
}

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct AlienTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
}

impl AlienTypeSignature {}
