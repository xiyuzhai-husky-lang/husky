use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TraitDeclarativeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclarativeSignatures,
}

impl TraitDeclarativeSignature {}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub fn trai_declarative_signature_from_decl(
    db: &dyn DeclarativeSignatureDb,
    decl: TraitDecl,
) -> DeclarativeSignatureResult<TraitDeclarativeSignature> {
    let expr_region = decl.expr_region(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    let implicit_parameters = ImplicitParameterDeclarativeSignatures::from_decl(
        decl.implicit_parameters(db),
        &declarative_term_region,
        declarative_term_menu,
    );
    Ok(TraitDeclarativeSignature::new(db, implicit_parameters))
}
