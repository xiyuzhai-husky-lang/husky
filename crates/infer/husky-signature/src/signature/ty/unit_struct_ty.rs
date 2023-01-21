use super::*;
use husky_word::Identifier;

#[salsa::tracked(jar = SignatureJar)]
pub fn unit_struct_ty_signature(
    db: &dyn SignatureDb,
    decl: UnitStructTypeDecl,
) -> UnitStructTypeSignature {
    let mut engine = SignatureTermEngine::new(db, decl.expr_region(db), None);
    UnitStructTypeSignature::new(
        db,
        ImplicitParameterSignatures::from_decl(decl.implicit_parameters(db), &mut engine),
        engine.finish(),
    )
}

#[salsa::tracked(jar = SignatureJar)]
pub struct UnitStructTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}

impl UnitStructTypeSignature {}
