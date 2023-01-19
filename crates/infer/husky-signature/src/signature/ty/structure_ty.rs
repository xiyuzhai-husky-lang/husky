use crate::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn structure_ty_signature(
    db: &dyn SignatureDb,
    decl: StructureTypeDecl,
) -> StructureTypeSignature {
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db));
    StructureTypeSignature::new(
        db,
        ImplicitParameterSignatures::from_decl(decl.implicit_parameters(db), &mut engine),
        engine.finish(),
    )
}

#[salsa::tracked(jar = SignatureJar)]
pub struct StructureTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}

impl StructureTypeSignature {}
