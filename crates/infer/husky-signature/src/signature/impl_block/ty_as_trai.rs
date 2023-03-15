use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_as_trai_im_signature(
    db: &dyn SignatureDb,
    decl: TypeAsTraitImplBlockDecl,
) -> SignatureResult<TypeAsTraitImplSignature> {
    let expr_region = decl.expr_region(db);
    let _signature_term_region = signature_term_region(db, expr_region);
    let _raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(TypeAsTraitImplSignature::new(
        db,
        // ImplicitParameterSignatureList::from_decl(decl.implicit_parameters(db), signature_term_region),
        // engine.finish(),
    ))
}

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct TypeAsTraitImplSignature {}
