use super::*;

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct TraitForTypeImplBlockSignature {}

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn trai_for_ty_impl_block_signature(
    db: &dyn SignatureDb,
    decl: TraitForTypeImplBlockDecl,
) -> SignatureResult<TraitForTypeImplBlockSignature> {
    let expr_region = decl.expr_region(db);
    let _signature_term_region = signature_term_region(db, expr_region);
    let _raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(TraitForTypeImplBlockSignature::new(
        db,
        // ImplicitParameterSignatureList::from_decl(decl.implicit_parameters(db), signature_term_region),
        // engine.finish(),
    ))
}
