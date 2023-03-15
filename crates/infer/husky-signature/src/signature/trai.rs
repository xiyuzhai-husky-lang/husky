use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn trai_signature_from_decl(
    db: &dyn SignatureDb,
    decl: TraitDecl,
) -> SignatureResult<TraitSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    let implicit_parameters = ImplicitParameterSignatures::from_decl(
        decl.implicit_parameters(db)?,
        &signature_term_region,
        raw_term_menu,
    );
    Ok(TraitSignature::new(db, implicit_parameters))
}

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct TraitSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
}

impl TraitSignature {}
