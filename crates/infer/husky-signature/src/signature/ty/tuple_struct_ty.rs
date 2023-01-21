use super::*;
use husky_word::Identifier;

#[salsa::tracked(jar = SignatureJar)]
pub fn tuple_struct_ty_signature(
    db: &dyn SignatureDb,
    decl: TupleStructTypeDecl,
) -> TupleStructTypeSignature {
    TupleStructTypeSignature::new(
        db,
        // ImplicitParameterSignatures::from_decl(decl.implicit_parameters(db), &mut engine),
        todo!(),
        todo!(),
    )
}

#[salsa::interned(jar = SignatureJar)]
pub struct TupleStructTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    #[return_ref]
    pub fields: Vec<TupleStructFieldSignature>,
}

impl TupleStructTypeSignature {}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct TupleStructFieldSignature {
    ty: Term,
}
