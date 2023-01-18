use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn structure_ty_signature(
    db: &dyn SignatureDb,
    decl: StructureTypeDecl,
) -> StructureTypeSignature {
    // implementation
    todo!()
}

#[salsa::tracked(jar = SignatureJar)]
pub struct StructureTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatureList,
}

impl StructureTypeSignature {}
