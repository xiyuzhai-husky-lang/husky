use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn enum_ty_signature(db: &dyn SignatureDb, decl: EnumTypeDecl) -> EnumTypeSignature {
    todo!()
}

#[salsa::tracked(jar = SignatureJar)]
pub struct EnumTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatureList,
}

impl EnumTypeSignature {}
