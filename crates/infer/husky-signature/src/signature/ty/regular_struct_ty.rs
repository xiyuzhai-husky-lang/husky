use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn regular_struct_ty_signature(
    db: &dyn SignatureDb,
    decl: RegularStructTypeDecl,
) -> RegularStructTypeSignature {
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db));
    RegularStructTypeSignature::new(
        db,
        ImplicitParameterSignatures::from_decl(decl.implicit_parameters(db), &mut engine),
        todo!(),
        engine.finish(),
    )
}

#[salsa::tracked(jar = SignatureJar)]
pub struct RegularStructTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    #[return_ref]
    pub fields: Vec<RegularStructFieldSignature>,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}

impl RegularStructTypeSignature {}

#[derive(Debug, PartialEq, Eq)]
pub struct RegularStructFieldSignature {
    ident: Identifier,
    ty: Term,
}
