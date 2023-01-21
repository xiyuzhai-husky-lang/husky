use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn morphism_signature(db: &dyn SignatureDb, decl: MorphismDecl) -> MorphismSignature {
    // implementation
    MorphismSignature::new(
        db,
        todo!(), // ImplicitParameterSignatures::from_decl(decl.implicit_parameters(db), &mut engine),
    )
}

#[salsa::interned(jar = SignatureJar)]
pub struct MorphismSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
}

impl MorphismSignature {}
