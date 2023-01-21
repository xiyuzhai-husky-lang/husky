use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub(crate) fn ty_as_trai_impl_block_signature(
    db: &dyn SignatureDb,
    decl: TypeAsTraitImplBlockDecl,
) -> TypeAsTraitImplBlockSignature {
    // implementation
    TypeAsTraitImplBlockSignature::new(
        db,
        // ImplicitParameterSignatureList::from_decl(decl.implicit_parameters(db), &mut engine),
        // engine.finish(),
    )
}

#[salsa::interned(jar = SignatureJar)]
pub struct TypeAsTraitImplBlockSignature {}
