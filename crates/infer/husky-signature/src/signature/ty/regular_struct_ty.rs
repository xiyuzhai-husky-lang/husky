use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn regular_struct_ty_signature(
    db: &dyn SignatureDb,
    decl: RegularStructTypeDecl,
) -> RegularStructTypeSignature {
    // implementation
    todo!()
}

#[salsa::tracked(jar = SignatureJar)]
pub struct RegularStructTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatureList,
    #[return_ref]
    pub fields: Vec<RegularStructFieldSignature>,
}

impl RegularStructTypeSignature {}

#[derive(Debug, PartialEq, Eq)]
pub struct RegularStructFieldSignature {
    ident: Identifier,
    ty: Term,
}
