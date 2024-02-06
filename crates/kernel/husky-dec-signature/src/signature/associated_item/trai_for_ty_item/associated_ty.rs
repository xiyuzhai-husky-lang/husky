use crate::*;

#[salsa::interned(db = DecSignatureDb, jar = DecSignatureJar)]
pub struct TraitForTypeAssocTypeDecTemplate {
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: DeclarativeTemplateParameterTemplates,
    pub ty_term: DecTerm,
}

impl TraitForTypeAssocTypeDecTemplate {
    pub(crate) fn from_decl(
        db: &::salsa::Db,
        path: TraitForTypeItemPath,
        decl: TraitForTypeAssocTypeSynDecl,
    ) -> DecSignatureResult<Self> {
        let syn_expr_region = decl.syn_expr_region(db);
        let declarative_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let declarative_term_menu = db
            .declarative_term_menu(syn_expr_region.toolchain(db))
            .unwrap();
        let template_parameters = DeclarativeTemplateParameterTemplates::from_decl(
            decl.template_parameters(db),
            declarative_term_region,
            declarative_term_menu,
        );
        let ty_term = declarative_term_region.expr_term(decl.ty_term_expr_idx(db))?;
        Ok(TraitForTypeAssocTypeDecTemplate::new(
            db,
            path,
            template_parameters,
            ty_term,
        ))
    }
}
