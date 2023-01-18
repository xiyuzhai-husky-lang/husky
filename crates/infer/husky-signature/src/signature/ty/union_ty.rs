use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn union_ty_signature(db: &dyn SignatureDb, decl: UnionTypeDecl) -> UnionTypeSignature {
    // implementation
    todo!()
}

#[salsa::tracked(jar = SignatureJar)]
pub struct UnionTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatureList,
}

impl UnionTypeSignature {}
