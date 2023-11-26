use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TraitDeclarativeSignatureTemplate {
    #[return_ref]
    pub template_parameters: DeclarativeTemplateParameterTemplates,
}

impl TraitDeclarativeSignatureTemplate {
    pub fn template_parameters_without_self_ty<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> &'a [DeclarativeTemplateParameter] {
        let template_parameters = self.template_parameters(db);
        debug_assert!(matches!(
            self.template_parameters(db)
                .last()
                .unwrap()
                .symbol()
                .index(db)
                .inner(),
            DeclarativeTermSymbolIndexInner::SelfType
        ));
        &template_parameters[..template_parameters.len() - 1]
    }
}

impl HasDeclarativeSignatureTemplate for TraitPath {
    type DeclarativeSignatureTemplate = TraitDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &::salsa::Db,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        trai_syn_declarative_signature_template(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub fn trai_syn_declarative_signature_template(
    db: &::salsa::Db,
    path: TraitPath,
) -> DeclarativeSignatureResult<TraitDeclarativeSignatureTemplate> {
    let decl = path.syn_decl(db)?;
    let syn_expr_region = decl.syn_expr_region(db);
    let declarative_term_region = declarative_term_region(db, syn_expr_region);
    let declarative_term_menu = db
        .declarative_term_menu(syn_expr_region.toolchain(db))
        .unwrap();
    let template_parameters = DeclarativeTemplateParameterTemplates::from_decl(
        decl.template_parameters(db),
        &declarative_term_region,
        declarative_term_menu,
    );
    Ok(TraitDeclarativeSignatureTemplate::new(
        db,
        template_parameters,
    ))
}
