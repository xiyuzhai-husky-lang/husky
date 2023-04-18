use super::*;

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub fn tuple_struct_ty_declarative_signature(
    db: &dyn DeclarativeSignatureDb,
    decl: TupleStructTypeDecl,
) -> DeclarativeSignatureResult<TupleStructTypeDeclarativeSignature> {
    let expr_region = decl.expr_region(db);
    let _declarative_term_region = declarative_term_region(db, expr_region);
    let _declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(TupleStructTypeDeclarativeSignature::new(
        db,
        // ImplicitParameterDeclarativeSignatures::from_decl(decl.implicit_parameters(db), declarative_term_region),
        todo!(),
        todo!(),
    ))
}

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TupleStructTypeDeclarativeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclarativeSignatures,
    #[return_ref]
    pub fields: Vec<TupleStructFieldSignature>,
}

impl TupleStructTypeDeclarativeSignature {}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct TupleStructFieldSignature {
    ty: DeclarativeTerm,
}
