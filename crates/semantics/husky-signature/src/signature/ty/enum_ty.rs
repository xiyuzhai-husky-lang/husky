use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn enum_ty_signature(
    db: &dyn SignatureDb,
    decl: EnumTypeDecl,
) -> SignatureResult<EnumTypeSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    let implicit_parameters = ImplicitParameterSignatures::from_decl(
        decl.implicit_parameters(db),
        &signature_term_region,
        raw_term_menu,
    );
    Ok(EnumTypeSignature::new(db, implicit_parameters))
}

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct EnumTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
}

impl EnumTypeSignature {}
