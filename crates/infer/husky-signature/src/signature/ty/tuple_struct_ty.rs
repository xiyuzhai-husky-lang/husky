use super::*;
use husky_word::Identifier;

#[salsa::tracked(jar = SignatureJar)]
pub fn tuple_struct_ty_signature(
    db: &dyn SignatureDb,
    decl: TupleStructTypeDecl,
) -> TupleStructTypeSignature {
    let mut engine = SignatureTermEngine::new(db, decl.expr_page(db), None);
    TupleStructTypeSignature::new(
        db,
        ImplicitParameterSignatures::from_decl(decl.implicit_parameters(db), &mut engine),
        todo!(),
        engine.finish(),
    )
}

#[salsa::tracked(jar = SignatureJar)]
pub struct TupleStructTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    #[return_ref]
    pub fields: Vec<TupleStructFieldSignature>,
    #[return_ref]
    pub term_sheet: SignatureTermSheet,
}

impl TupleStructTypeSignature {}

#[derive(Debug, PartialEq, Eq)]
pub struct TupleStructFieldSignature {
    ty: Term,
}
