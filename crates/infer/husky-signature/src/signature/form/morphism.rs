use crate::*;

#[salsa::tracked(jar = SignatureJar,return_ref)]
pub fn morphism_signature(
    db: &dyn SignatureDb,
    decl: MorphismDecl,
) -> SignatureOutcome<MorphismSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    let implicit_parameters = ImplicitParameterSignatures::from_decl(
        decl.implicit_parameters(db),
        &signature_term_region,
        term_menu,
    );
    Success(MorphismSignature::new(db, implicit_parameters))
}

#[salsa::interned(jar = SignatureJar)]
pub struct MorphismSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
}

impl MorphismSignature {}
