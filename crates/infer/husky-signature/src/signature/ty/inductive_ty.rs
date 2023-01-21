use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn inductive_ty_signature(
    db: &dyn SignatureDb,
    decl: InductiveTypeDecl,
) -> InductiveTypeSignature {
    InductiveTypeSignature::new(
        db,
        todo!(), // ImplicitParameterSignatures::from_decl(decl.implicit_parameters(db), &mut engine),
    )
}

#[salsa::interned(jar = SignatureJar)]
pub struct InductiveTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
}

impl InductiveTypeSignature {}
