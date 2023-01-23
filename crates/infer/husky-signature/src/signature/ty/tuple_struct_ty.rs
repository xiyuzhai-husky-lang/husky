use super::*;
use husky_word::Identifier;

#[salsa::tracked(jar = SignatureJar,return_ref)]
pub fn tuple_struct_ty_signature(
    db: &dyn SignatureDb,
    decl: TupleStructTypeDecl,
) -> SignatureResult<TupleStructTypeSignature> {
    let expr_region = decl.expr_region(db);
    let signature_term_region = signature_term_region(db, expr_region);
    let term_menu = db.term_menu(expr_region.toolchain(db)).as_ref().unwrap();
    Ok(TupleStructTypeSignature::new(
        db,
        // ImplicitParameterSignatures::from_decl(decl.implicit_parameters(db), signature_term_region),
        todo!(),
        todo!(),
    ))
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
