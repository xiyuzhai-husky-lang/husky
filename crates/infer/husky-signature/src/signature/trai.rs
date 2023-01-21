use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn trai_signature(db: &dyn SignatureDb, decl: TraitDecl) -> TraitSignature {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    let implicit_parameters = ImplicitParameterSignatures::from_decl(
        decl.implicit_parameters(db),
        &signature_term_region,
        term_menu,
    );
    TraitSignature::new(db, implicit_parameters)
}

#[salsa::interned(jar = SignatureJar)]
pub struct TraitSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
}

impl TraitSignature {}
