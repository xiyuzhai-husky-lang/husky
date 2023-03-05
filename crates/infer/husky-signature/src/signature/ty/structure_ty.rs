use crate::*;

#[salsa::tracked(jar = SignatureJar,return_ref)]
pub fn structure_ty_signature(
    db: &dyn SignatureDb,
    decl: StructureTypeDecl,
) -> SignatureResult<StructureTypeSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    let implicit_parameters = ImplicitParameterSignatures::from_decl(
        decl.implicit_parameters(db)?,
        &signature_term_region,
        raw_term_menu,
    );
    Ok(StructureTypeSignature::new(db, implicit_parameters))
}

#[salsa::tracked(db = SignatureDb, jar = SignatureJar)]
pub struct StructureTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
}

impl StructureTypeSignature {}
