use super::*;
use husky_entity_path::path::assoc_item::trai_for_ty_item::TraitForTypeItemPath;

#[salsa::interned]
pub struct TraitForTypeAssocTypeDecTemplate {
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: DecTemplateParameters,
    pub ty_term: DecTerm,
}

impl TraitForTypeAssocTypeDecTemplate {
    pub(crate) fn from_decl(
        db: &::salsa::Db,
        path: TraitForTypeItemPath,
        decl: TraitForTypeAssocTypeSynDecl,
    ) -> DecSignatureResult<Self> {
        use husky_dec_term::jar::DecTermDb;

        let syn_expr_region = decl.syn_expr_region(db);
        let dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let dec_term_menu = db.dec_term_menu(syn_expr_region.toolchain(db)).unwrap();
        let template_parameters = DecTemplateParameters::from_decl(
            decl.template_parameters(db),
            dec_term_region,
            dec_term_menu,
        );
        let ty_term = dec_term_region.expr_term(decl.ty_term_expr_idx(db))?;
        Ok(TraitForTypeAssocTypeDecTemplate::new(
            db,
            path,
            template_parameters,
            ty_term,
        ))
    }
}
