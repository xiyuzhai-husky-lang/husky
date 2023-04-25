use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
pub struct ExternTypeDeclarativeSignature {}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub fn extern_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    decl: ExternTypeDecl,
) -> DeclarativeSignatureResult<ExternDeclarativeSignatureTemplate> {
    let expr_region = decl.expr_region(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(ExternDeclarativeSignatureTemplate::new(
        db,
        ImplicitParameterDeclarativeSignatures::from_decl(
            decl.implicit_parameters(db),
            &declarative_term_region,
            declarative_term_menu,
        ),
    ))
}

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct ExternDeclarativeSignatureTemplate {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclarativeSignatures,
}

impl ExternDeclarativeSignatureTemplate {}
