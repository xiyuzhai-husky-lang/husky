use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn inductive_ty_signature(
    db: &dyn SignatureDb,
    decl: InductiveTypeDecl,
) -> InductiveTypeSignature {
    // implementation
    todo!()
}

#[salsa::tracked(jar = SignatureJar)]
pub struct InductiveTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatureList,
}

impl InductiveTypeSignature {}
