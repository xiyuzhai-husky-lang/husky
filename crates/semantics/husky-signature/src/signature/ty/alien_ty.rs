use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn alien_ty_signature(
    db: &dyn SignatureDb,
    decl: ExternTypeDecl,
) -> SignatureResult<ExternTypeSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(ExternTypeSignature::new(
        db,
        ImplicitParameterSignatures::from_decl(
            decl.implicit_parameters(db),
            &signature_term_region,
            raw_term_menu,
        ),
    ))
}

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct ExternTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
}

impl ExternTypeSignature {}
