use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TypeAssociatedFnDeclarativeSignatureTemplate {
    /// the term for `Self`
    /// not necessarily equal to the type of `self`
    ///
    /// we don't use self_ty_arguments because it's not determined for declarative terms
    pub self_ty: DeclarativeTerm,
    #[return_ref]
    pub template_parameters: DeclarativeTemplateParameterTemplates,
    #[return_ref]
    pub parenic_parameters: DeclarativeParenicParameters,
    pub return_ty: DeclarativeTerm,
}

impl HasDeclarativeSignatureTemplate for TypeAssociatedFnSynDecl {
    type DeclarativeSignatureTemplate = TypeAssociatedFnDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<TypeAssociatedFnDeclarativeSignatureTemplate> {
        ty_associated_fn_declarative_signature_template(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn ty_associated_fn_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    decl: TypeAssociatedFnSynDecl,
) -> DeclarativeSignatureResult<TypeAssociatedFnDeclarativeSignatureTemplate> {
    let syn_expr_region = decl.syn_expr_region(db);
    let expr_region_data = syn_expr_region.data(db);
    let declarative_term_region = declarative_term_region(db, syn_expr_region);
    let declarative_term_menu = db
        .declarative_term_menu(syn_expr_region.toolchain(db))
        .unwrap();
    let impl_block = decl
        .path(db)
        .impl_block(db)
        .declarative_signature_template(db)?;
    let self_ty = impl_block.ty(db);
    let template_parameters = DeclarativeTemplateParameterTemplates::from_decl(
        decl.template_parameters(db),
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
    Ok(TypeAssociatedFnDeclarativeSignatureTemplate::new(
        db,
        self_ty,
        template_parameters,
        parenic_parameters,
        return_ty,
    ))
}
