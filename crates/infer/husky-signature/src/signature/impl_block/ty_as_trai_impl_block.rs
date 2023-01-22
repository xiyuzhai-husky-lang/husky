use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_as_trai_impl_block_signature(
    db: &dyn SignatureDb,
    decl: TypeAsTraitImplBlockDecl,
) -> TypeAsTraitImplBlockSignature {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    TypeAsTraitImplBlockSignature::new(
        db,
        // ImplicitParameterSignatureList::from_decl(decl.implicit_parameters(db), signature_term_region),
        // engine.finish(),
    )
}

#[salsa::interned(jar = SignatureJar)]
pub struct TypeAsTraitImplBlockSignature {}
