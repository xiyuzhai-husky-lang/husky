use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TraitForTypeMethodFnDeclarativeSignatureTemplate {
    pub self_ty: DeclarativeTerm,
    #[return_ref]
    pub template_parameters: DeclarativeTemplateParameterTemplates,
    pub self_value_parameter: DeclarativeRitchieRegularParameter,
    /// parenate is a word I coined
    ///
    /// it means things that should be parenthesized
    #[return_ref]
    pub parenate_parameters: DeclarativeParenateParameters,
    pub return_ty: DeclarativeTerm,
}

impl TraitForTypeMethodFnDeclarativeSignatureTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        decl: TraitForTypeMethodFnSynDecl,
    ) -> DeclarativeSignatureResult<Self> {
        let self_ty = decl
            .path(db)
            .impl_block(db)
            .declarative_signature_template(db)?
            .self_ty(db)
            .term();
        let self_value_parameter = DeclarativeRitchieRegularParameter::new(
            match decl.self_value_parameter(db) {
                Some(self_value_parameter) => todo!(),
                None => TermContract::None,
            },
            self_ty,
        );
        let syn_expr_region = decl.syn_expr_region(db);
        let expr_region_data = syn_expr_region.data(db);
        let declarative_term_region = declarative_term_region(db, syn_expr_region);
        let declarative_term_menu = db
            .declarative_term_menu(syn_expr_region.toolchain(db))
            .unwrap();
        let template_parameters = DeclarativeTemplateParameterTemplates::from_decl(
            decl.template_parameters(db),
            declarative_term_region,
            declarative_term_menu,
        );
        let parenate_parameters = DeclarativeParenateParameters::from_decl(
            decl.parenate_parameters(db),
            expr_region_data,
            declarative_term_region,
        )?;
        let return_ty = match decl.return_ty(db) {
            Some(return_ty) => declarative_term_region.expr_term(return_ty.syn_expr_idx())?,
            None => declarative_term_menu.unit(),
        };
        Ok(TraitForTypeMethodFnDeclarativeSignatureTemplate::new(
            db,
            self_ty,
            template_parameters,
            self_value_parameter,
            parenate_parameters,
            return_ty,
        ))
    }
}
