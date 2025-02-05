use super::*;
use husky_entity_path::path::major_item::trai::TraitPath;

#[salsa::interned]
pub struct TraitDecTemplate {
    #[return_ref]
    pub template_parameters: DecTemplateParameters,
}

impl TraitDecTemplate {
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
            DecTermSymbolIndexImpl::SelfType
        ));
        &template_parameters[..template_parameters.len() - 1]
    }
}

impl HasDecTemplate for TraitPath {
    type DecTemplate = TraitDecTemplate;

    fn dec_template(self, db: &::salsa::Db) -> DecSignatureResult<Self::DecTemplate> {
        trai_syn_dec_template(db, self)
    }
}

#[salsa::tracked]
pub fn trai_syn_dec_template(
    db: &::salsa::Db,
    path: TraitPath,
) -> DecSignatureResult<TraitDecTemplate> {
    let decl = path.syn_decl(db)?;
    let syn_expr_region = decl.syn_expr_region(db);
    let dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
    let dec_term_menu = db.dec_term_menu(syn_expr_region.toolchain(db)).unwrap();
    let template_parameters = DecTemplateParameters::from_decl(
        decl.template_parameters(db),
        &dec_term_region,
        dec_term_menu,
    );
    Ok(TraitDecTemplate::new(db, template_parameters))
}
