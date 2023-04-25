use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
pub struct StructureTypeDeclarativeSignature {}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub fn structure_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    decl: StructureTypeDecl,
) -> DeclarativeSignatureResult<StructureDeclarativeSignatureTemplate> {
    let expr_region = decl.expr_region(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    let implicit_parameters = ImplicitParameterDeclarativeSignatures::from_decl(
        decl.implicit_parameters(db),
        &declarative_term_region,
        declarative_term_menu,
    );
    Ok(StructureDeclarativeSignatureTemplate::new(
        db,
        implicit_parameters,
    ))
}

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct StructureDeclarativeSignatureTemplate {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclarativeSignatures,
}

impl StructureDeclarativeSignatureTemplate {}
