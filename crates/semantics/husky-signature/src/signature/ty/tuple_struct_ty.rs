use super::*;

#[salsa::tracked(jar = SignatureJar)]
pub fn tuple_struct_ty_signature(
    db: &dyn SignatureDb,
    decl: TupleStructTypeDecl,
) -> SignatureResult<TupleStructTypeSignature> {
    let expr_region = decl.expr_region(db);
    let _signature_term_region = signature_term_region(db, expr_region);
    let _raw_term_menu = db.raw_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(TupleStructTypeSignature::new(
        db,
        // ImplicitParameterSignatures::from_decl(decl.implicit_parameters(db), signature_term_region),
        todo!(),
        todo!(),
    ))
}

#[salsa::interned(db = SignatureDb, jar = SignatureJar)]
pub struct TupleStructTypeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    #[return_ref]
    pub fields: Vec<TupleStructFieldSignature>,
}

impl TupleStructTypeSignature {}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct TupleStructFieldSignature {
    ty: RawTerm,
}
