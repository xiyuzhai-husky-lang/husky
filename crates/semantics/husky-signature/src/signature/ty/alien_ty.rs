use super::*;

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub fn alien_ty_declarative_signature(
    db: &dyn DeclarativeSignatureDb,
    decl: ExternTypeDecl,
) -> DeclarativeSignatureResult<ExternTypeDeclarativeSignature> {
    let expr_region = decl.expr_region(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(ExternTypeDeclarativeSignature::new(
        db,
        ImplicitParameterSignatures::from_decl(
            decl.implicit_parameters(db),
            &declarative_term_region,
            declarative_term_menu,
        ),
    ))
}

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct ExternTypeDeclarativeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
}

impl ExternTypeDeclarativeSignature {}
