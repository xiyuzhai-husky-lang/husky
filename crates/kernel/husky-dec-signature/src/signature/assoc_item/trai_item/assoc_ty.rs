use crate::*;
use husky_entity_path::path::assoc_item::trai_item::TraitItemPath;

#[salsa::interned]
pub struct TraitAssocTypeDecTemplate {
    pub path: TraitItemPath,
    #[return_ref]
    pub template_parameters: DecTemplateParameters,
    pub ty_term: Option<DecTerm>,
}

impl TraitAssocTypeDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        path: TraitItemPath,
        decl: TraitAssocTypeSynDecl,
    ) -> DecSignatureResult<TraitAssocTypeDecTemplate> {
        let syn_expr_region = decl.syn_expr_region(db);
        let dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let dec_term_menu = db.dec_term_menu(syn_expr_region.toolchain(db)).unwrap();
        let template_parameters = DecTemplateParameters::from_decl(
            decl.template_parameters(db),
            dec_term_region,
            dec_term_menu,
        );
        let ty_term = match decl.ty_term(db) {
            Some(ty_term) => Some(dec_term_region.expr_term(ty_term)?),
            None => None,
        };
        Ok(TraitAssocTypeDecTemplate::new(
            db,
            path,
            template_parameters,
            ty_term,
        ))
    }
}
