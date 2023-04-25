use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
pub struct TupleStructTypeDeclarativeSignature {}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub fn tuple_struct_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    decl: TupleStructTypeDecl,
) -> DeclarativeSignatureResult<TupleStructDeclarativeSignatureTemplate> {
    let expr_region = decl.expr_region(db);
    let _declarative_term_region = declarative_term_region(db, expr_region);
    let _declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(TupleStructDeclarativeSignatureTemplate::new(
        db,
        // ImplicitParameterDeclarativeSignatureTemplates::from_decl(decl.implicit_parameters(db), declarative_term_region),
        todo!(),
        todo!(),
    ))
}

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TupleStructDeclarativeSignatureTemplate {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclarativeSignatures,
    #[return_ref]
    pub fields: Vec<TupleStructFieldSignature>,
}

impl TupleStructDeclarativeSignatureTemplate {}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct TupleStructFieldSignature {
    ty: DeclarativeTerm,
}
