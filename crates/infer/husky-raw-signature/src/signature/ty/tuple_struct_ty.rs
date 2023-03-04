use super::*;
use husky_word::Identifier;

#[salsa::tracked(jar = RawSignatureJar,return_ref)]
pub fn tuple_struct_ty_raw_signature(
    db: &dyn RawSignatureDb,
    decl: TupleStructTypeDecl,
) -> RawSignatureResult<TupleStructTypeRawSignature> {
    let expr_region = decl.expr_region(db);
    let raw_signature_term_region = raw_signature_term_region(db, expr_region);
    let raw_term_menu = db
        .raw_term_menu(expr_region.toolchain(db))
        .as_ref()
        .unwrap();
    Ok(TupleStructTypeRawSignature::new(
        db,
        // ImplicitParameterRawSignatures::from_decl(decl.implicit_parameters(db), raw_signature_term_region),
        todo!(),
        todo!(),
    ))
}

#[salsa::interned(db = RawSignatureDb, jar = RawSignatureJar)]
pub struct TupleStructTypeRawSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterRawSignatures,
    #[return_ref]
    pub fields: Vec<TupleStructFieldRawSignature>,
}

impl TupleStructTypeRawSignature {}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct TupleStructFieldRawSignature {
    ty: RawTerm,
}
