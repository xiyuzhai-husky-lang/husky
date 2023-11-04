use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct FnFugitiveDeclarativeSignatureTemplate {
    #[return_ref]
    pub template_parameters: DeclarativeTemplateParameterTemplates,
    #[return_ref]
    pub parenate_parameters: DeclarativeParenateParameters,
    pub return_ty: DeclarativeTerm,
}

impl FnFugitiveDeclarativeSignatureTemplate {
    pub(super) fn from_decl(
        db: &dyn DeclarativeSignatureDb,
        decl: FunctionFnSynDecl,
    ) -> DeclarativeSignatureResult<Self> {
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
            Some(return_ty) => declarative_term_region.expr_term(return_ty.expr())?,
            None => declarative_term_menu.unit(),
        };
        Ok(FnFugitiveDeclarativeSignatureTemplate::new(
            db,
            template_parameters,
            parenate_parameters,
            return_ty,
        ))
    }
}
