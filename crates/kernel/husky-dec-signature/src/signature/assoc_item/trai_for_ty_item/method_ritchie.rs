use super::*;
use husky_syn_decl::decl::assoc_item::trai_for_ty_item::method_ritchie::TraitForTypeMethodRitchieSynDecl;

#[salsa::interned]
pub struct TraitForTypeMethodRitchieDecTemplate {
    pub self_ty: DecTerm,
    #[return_ref]
    pub template_parameters: DecTemplateParameters,
    pub self_value_parameter: DeclarativeRitchieSimpleParameter,
    #[return_ref]
    pub parenate_parameters: DeclarativeParenateParameters,
    pub return_ty: DecTerm,
}

impl TraitForTypeMethodRitchieDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        decl: TraitForTypeMethodRitchieSynDecl,
    ) -> DecSignatureResult<Self> {
        let self_ty = decl
            .path(db)
            .impl_block(db)
            .dec_template(db)?
            .self_ty(db)
            .term();
        let self_value_parameter = DeclarativeRitchieSimpleParameter::new(
            match decl.self_value_parameter(db) {
                Some(self_value_parameter) => todo!(),
                None => Contract::Pure,
            },
            self_ty,
        );
        let syn_expr_region = decl.syn_expr_region(db);
        let syn_expr_region_data = syn_expr_region.data(db);
        let dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let dec_term_menu = db.dec_term_menu(syn_expr_region.toolchain(db)).unwrap();
        let template_parameters = DecTemplateParameters::from_decl(
            decl.template_parameters(db),
            dec_term_region,
            dec_term_menu,
        );
        let parenate_parameters = DeclarativeParenateParameters::from_decl(
            decl.parenate_parameters(db),
            syn_expr_region_data,
            dec_term_region,
        )?;
        let return_ty = match decl.return_ty(db) {
            Some(return_ty) => dec_term_region.expr_term(return_ty.syn_expr_idx())?,
            None => dec_term_menu.unit(),
        };
        Ok(TraitForTypeMethodRitchieDecTemplate::new(
            db,
            self_ty,
            template_parameters,
            self_value_parameter,
            parenate_parameters,
            return_ty,
        ))
    }
}
