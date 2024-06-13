use crate::*;
use husky_entity_path::path::assoc_item::trai_item::TraitItemPath;
use husky_syn_decl::decl::assoc_item::trai_item::assoc_ty::TraitAssocTypeSynDecl;

#[salsa::interned]
pub struct TraitAssocTypeDecTemplate {
    pub path: TraitItemPath,
    #[return_ref]
    pub template_parameters: DecTemplateParameters,
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
        Ok(TraitAssocTypeDecTemplate::new(
            db,
            path,
            template_parameters,
        ))
    }
}
