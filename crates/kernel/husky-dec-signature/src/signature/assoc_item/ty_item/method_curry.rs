use super::*;

#[salsa::interned]
pub struct TypeMethodCurryDecTemplate {
    pub path: TypeItemPath,
    // todo: formal method, method that is not a function pointer
    #[return_ref]
    pub template_parameters: DecTemplateParameters,
    pub self_value_parameter: DeclarativeRitchieSimpleParameter,
    #[return_ref]
    pub parenate_parameters: DeclarativeParenateParameters,
    pub return_ty: DecTerm,
}

impl TypeMethodCurryDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        path: TypeItemPath,
        decl: TypeMethodRitchieSynDecl,
    ) -> DecSignatureResult<Self> {
        // todo: overhaul
        // the following is blindly copied from method fn
        let syn_expr_region = decl.syn_expr_region(db);
        let expr_region_data = syn_expr_region.data(db);
        let dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let self_value_parameter = DeclarativeRitchieSimpleParameter::new(
            match decl.self_value_parameter(db) {
                Some(self_value_parameter) => todo!(),
                None => Contract::Pure,
            },
            decl.impl_block_path(db).dec_template(db)?.ty(db),
        );
        let dec_term_menu = db.dec_term_menu(syn_expr_region.toolchain(db)).unwrap();
        let template_parameters = DecTemplateParameters::from_decl(
            decl.template_parameters(db),
            dec_term_region,
            dec_term_menu,
        );
        let parenate_parameters = DeclarativeParenateParameters::from_decl(
            decl.parenate_parameters(db),
            expr_region_data,
            dec_term_region,
        )?;
        let return_ty = match decl.return_ty(db) {
            Some(return_ty) => dec_term_region.expr_term(return_ty.syn_expr_idx())?,
            None => dec_term_menu.unit(),
        };
        Ok(TypeMethodCurryDecTemplate::new(
            db,
            path,
            template_parameters,
            self_value_parameter,
            parenate_parameters,
            return_ty,
        ))
    }
}
