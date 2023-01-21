use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn union_ty_signature(db: &dyn SignatureDb, decl: UnionTypeDecl) -> UnionTypeSignature {
    UnionTypeSignature::new(
        db,
        todo!(), // ImplicitParameterSignatures::from_decl(decl.implicit_parameters(db), &mut engine),
    )
}

#[salsa::interned(jar = SignatureJar)]
pub struct UnionTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
}

impl UnionTypeSignature {}
