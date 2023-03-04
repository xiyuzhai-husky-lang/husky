use super::*;

#[salsa::tracked(jar = RawSignatureJar,return_ref)]
pub fn alien_ty_raw_signature(
    db: &dyn RawSignatureDb,
    decl: ExternTypeDecl,
) -> RawSignatureResult<ExternTypeRawSignature> {
    let expr_region = decl.expr_region(db);
    let raw_signature_term_region = raw_signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    Ok(ExternTypeRawSignature::new(
        db,
        ImplicitParameterRawSignatures::from_decl(
            decl.implicit_parameters(db)?,
            &raw_signature_term_region,
            term_menu,
        ),
    ))
}

#[salsa::interned(db = RawSignatureDb, jar = RawSignatureJar)]
pub struct ExternTypeRawSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterRawSignatures,
}

impl ExternTypeRawSignature {}
