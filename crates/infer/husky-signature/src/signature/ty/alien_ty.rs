use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn alien_ty_signature(db: &dyn SignatureDb, decl: AlienTypeDecl) -> AlienTypeSignature {
    // implementation
    AlienTypeSignature::new(
        db,
        todo!(), // ImplicitParameterSignatures::from_decl(decl.implicit_parameters(db), &mut engine),
                 // engine.finish(),
    )
}

#[salsa::interned(jar = SignatureJar)]
pub struct AlienTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
}

impl AlienTypeSignature {}
