use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TraitAssociatedFnDeclarativeSignatureTemplate {
    #[return_ref]
    pub template_parameters: DeclarativeTemplateParameterTemplates,
    #[return_ref]
    pub parenate_parameters: DeclarativeParenateParameters,
    pub return_ty: DeclarativeTerm,
}

impl TraitAssociatedFnDeclarativeSignatureTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        decl: TraitAssociatedFnSynDecl,
    ) -> DeclarativeSignatureResult<TraitAssociatedFnDeclarativeSignatureTemplate> {
        let syn_expr_region = decl.syn_expr_region(db);
        let syn_expr_region_data = syn_expr_region.data(db);
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
            syn_expr_region_data,
            declarative_term_region,
        )?;
        let return_ty = match decl.return_ty(db) {
            Some(return_ty) => declarative_term_region.expr_term(return_ty.syn_expr_idx())?,
            None => declarative_term_menu.unit(),
        };
        Ok(TraitAssociatedFnDeclarativeSignatureTemplate::new(
            db,
            template_parameters,
            parenate_parameters,
            return_ty,
        ))
    }
}
