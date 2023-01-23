use super::*;

#[salsa::tracked(jar = SignatureJar,return_ref)]
pub fn union_ty_signature(
    db: &dyn SignatureDb,
    decl: UnionTypeDecl,
) -> SignatureResult<UnionTypeSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    let implicit_parameters = ImplicitParameterSignatures::from_decl(
        decl.implicit_parameters(db),
        &signature_term_region,
        term_menu,
    );
    Ok(UnionTypeSignature::new(db, implicit_parameters))
}

#[salsa::interned(jar = SignatureJar)]
pub struct UnionTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
}

impl UnionTypeSignature {}
