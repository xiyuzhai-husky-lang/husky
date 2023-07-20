use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TraitForTypeAssociatedTypeDeclarativeSignatureTemplate {
    #[return_ref]
    pub generic_parameters: DeclarativeGenericParameterTemplates,
    pub ty_term: DeclarativeTerm,
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn trai_for_ty_associated_ty_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    decl: TraitForTypeAssociatedTypeDecl,
) -> DeclarativeSignatureResult<TraitForTypeAssociatedTypeDeclarativeSignatureTemplate> {
    let expr_region = decl.expr_region(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    let generic_parameters = DeclarativeGenericParameterTemplates::from_decl(
        decl.generic_parameters(db),
        declarative_term_region,
        declarative_term_menu,
    );
    let ty_term = declarative_term_region.expr_term(decl.ty_term_expr_idx(db))?;
    Ok(TraitForTypeAssociatedTypeDeclarativeSignatureTemplate::new(
        db,
        generic_parameters,
        ty_term,
    ))
}
