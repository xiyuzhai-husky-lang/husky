use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclarativeSignatureDb)]
pub struct EnumTypeDeclarativeSignature {}

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureTemplateJar)]
pub struct EnumTypeDeclarativeSignatureTemplate {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclarativeSignatureTemplates,
}

#[salsa::tracked(jar = DeclarativeSignatureTemplateJar)]
pub fn enum_ty_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    decl: EnumTypeDecl,
) -> DeclarativeSignatureResult<EnumTypeDeclarativeSignatureTemplate> {
    let expr_region = decl.expr_region(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    let implicit_parameters = ImplicitParameterDeclarativeSignatureTemplates::from_decl(
        decl.implicit_parameters(db),
        &declarative_term_region,
        declarative_term_menu,
    );
    Ok(EnumTypeDeclarativeSignatureTemplate::new(
        db,
        implicit_parameters,
    ))
}

impl EnumTypeDeclarativeSignatureTemplate {}
