use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn trai_signature(db: &dyn SignatureDb, decl: TraitDecl) -> TraitSignature {
    // implementation
    TraitSignature::new(
        db,
        todo!(), // ImplicitParameterSignatures::from_decl(decl.implicit_parameters(db), &mut engine),
    )
}

#[salsa::interned(jar = SignatureJar)]
pub struct TraitSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
}

impl TraitSignature {}
