use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn enum_ty_signature(db: &dyn SignatureDb, decl: EnumTypeDecl) -> EnumTypeSignature {
    EnumTypeSignature::new(
        db,
        todo!(), // ImplicitParameterSignatures::from_decl(decl.implicit_parameters(db), &mut engine),
    )
}

#[salsa::interned(jar = SignatureJar)]
pub struct EnumTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
}

impl EnumTypeSignature {}
