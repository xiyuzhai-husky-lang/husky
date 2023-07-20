use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TypeAssociatedFnDeclarativeSignatureTemplate {
    /// the term for `Self`
    /// not necessarily equal to the type of `self`
    ///
    /// we don't use self_ty_arguments because it's not determined for declarative terms
    pub self_ty: DeclarativeTerm,
    #[return_ref]
    pub generic_parameters: DeclarativeGenericParameterTemplates,
    #[return_ref]
    pub explicit_parameters: DeclarativeParenicParameters,
    pub return_ty: DeclarativeTerm,
}

impl HasDeclarativeSignatureTemplate for TypeAssociatedFnDecl {
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
    decl: TypeAssociatedFnDecl,
) -> DeclarativeSignatureResult<TypeAssociatedFnDeclarativeSignatureTemplate> {
    let expr_region = decl.expr_region(db);
    let expr_region_data = expr_region.data(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    let impl_block = decl
        .path(db)
        .impl_block(db)
        .declarative_signature_template(db)?;
    let self_ty = impl_block.ty(db);
    let generic_parameters = DeclarativeGenericParameterTemplates::from_decl(
        decl.generic_parameters(db),
        declarative_term_region,
        declarative_term_menu,
    );
    let explicit_parameters = DeclarativeParenicParameters::from_decl(
        decl.explicit_parameters(db),
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
        generic_parameters,
        explicit_parameters,
        return_ty,
    ))
}
