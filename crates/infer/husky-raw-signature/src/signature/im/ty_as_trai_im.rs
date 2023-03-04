use super::*;

#[salsa::tracked(jar = RawSignatureJar,return_ref)]
pub(crate) fn ty_as_trai_im_raw_signature(
    db: &dyn RawSignatureDb,
    decl: TypeAsTraitImplDecl,
) -> RawSignatureResult<TypeAsTraitImplRawSignature> {
    let expr_region = decl.expr_region(db);
    let raw_signature_term_region = raw_signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    Ok(TypeAsTraitImplRawSignature::new(
        db,
        // ImplicitParameterRawSignatureList::from_decl(decl.implicit_parameters(db), raw_signature_term_region),
        // engine.finish(),
    ))
}

#[salsa::interned(db = RawSignatureDb, jar = RawSignatureJar)]
pub struct TypeAsTraitImplRawSignature {}
