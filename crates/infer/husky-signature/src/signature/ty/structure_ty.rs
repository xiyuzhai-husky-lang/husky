use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn structure_ty_signature(
    db: &dyn SignatureDb,
    decl: StructureTypeDecl,
) -> StructureTypeSignature {
    StructureTypeSignature::new(
        db,
        todo!(), // ImplicitParameterSignatures::from_decl(decl.implicit_parameters(db), &mut engine),
                 // engine.finish(),
    )
}

#[salsa::tracked(jar = SignatureJar)]
pub struct StructureTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
}

impl StructureTypeSignature {}
