use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn enum_ty_signature(db: &dyn SignatureDb, decl: EnumTypeDecl) -> EnumTypeSignature {
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db), None);
    EnumTypeSignature::new(
        db,
        ImplicitParameterSignatures::from_decl(decl.implicit_parameters(db), &mut engine),
        engine.finish(),
    )
}

#[salsa::tracked(jar = SignatureJar)]
pub struct EnumTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}

impl EnumTypeSignature {}
