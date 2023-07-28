use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TraitForTypeAssociatedTypeDeclarativeSignatureTemplate {
    #[return_ref]
    pub template_parameters: DeclarativeTemplateParameterTemplates,
    pub ty_term: DeclarativeTerm,
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn trai_for_ty_associated_ty_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    decl: TraitForTypeAssociatedTypeSynDecl,
) -> DeclarativeSignatureResult<TraitForTypeAssociatedTypeDeclarativeSignatureTemplate> {
    let syn_expr_region = decl.syn_expr_region(db);
    let declarative_term_region = declarative_term_region(db, syn_expr_region);
    let declarative_term_menu = db
        .declarative_term_menu(syn_expr_region.toolchain(db))
        .unwrap();
    let template_parameters = DeclarativeTemplateParameterTemplates::from_decl(
        decl.template_parameters(db),
        declarative_term_region,
        declarative_term_menu,
    );
    let ty_term = declarative_term_region.expr_term(decl.ty_term_expr_idx(db))?;
    Ok(TraitForTypeAssociatedTypeDeclarativeSignatureTemplate::new(
        db,
        template_parameters,
        ty_term,
    ))
}
