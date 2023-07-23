use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct FnDeclarativeSignatureTemplate {
    #[return_ref]
    pub generic_parameters: DeclarativeGenericParameterTemplates,
    #[return_ref]
    pub parenic_parameters: DeclarativeParenicParameters,
    pub return_ty: DeclarativeTerm,
}

impl HasDeclarativeSignatureTemplate for FnSynDecl {
    type DeclarativeSignatureTemplate = FnDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        fn_declarative_signature_template(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub fn fn_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    decl: FnSynDecl,
) -> DeclarativeSignatureResult<FnDeclarativeSignatureTemplate> {
    let syn_expr_region = decl.syn_expr_region(db);
    let expr_region_data = syn_expr_region.data(db);
    let declarative_term_region = declarative_term_region(db, syn_expr_region);
    let declarative_term_menu = db
        .declarative_term_menu(syn_expr_region.toolchain(db))
        .unwrap();
    let generic_parameters = DeclarativeGenericParameterTemplates::from_decl(
        decl.generic_parameters(db),
        declarative_term_region,
        declarative_term_menu,
    );
    let parenic_parameters = DeclarativeParenicParameters::from_decl(
        decl.parenic_parameters(db),
        expr_region_data,
        declarative_term_region,
    )?;
    let return_ty = match decl.return_ty(db) {
        Some(return_ty) => declarative_term_region.expr_term(return_ty.expr())?,
        None => declarative_term_menu.unit(),
    };
    Ok(FnDeclarativeSignatureTemplate::new(
        db,
        generic_parameters,
        parenic_parameters,
        return_ty,
    ))
}

impl FnDeclarativeSignatureTemplate {}
