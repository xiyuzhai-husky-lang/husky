use super::*;
use husky_word::Identifier;

#[salsa::tracked(jar = RawSignatureJar,return_ref)]
pub fn unit_struct_ty_raw_signature(
    db: &dyn RawSignatureDb,
    decl: UnitStructTypeDecl,
) -> RawSignatureResult<UnitStructTypeRawSignature> {
    let expr_region = decl.expr_region(db);
    let raw_signature_term_region = raw_signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    let implicit_parameters = ImplicitParameterRawSignatures::from_decl(
        decl.implicit_parameters(db)?,
        &raw_signature_term_region,
        term_menu,
    );
    Ok(UnitStructTypeRawSignature::new(db, implicit_parameters))
}

#[salsa::interned(db = RawSignatureDb, jar = RawSignatureJar)]
pub struct UnitStructTypeRawSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterRawSignatures,
}

impl UnitStructTypeRawSignature {}
