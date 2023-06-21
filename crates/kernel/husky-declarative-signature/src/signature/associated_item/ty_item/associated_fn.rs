use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TypeAssociatedFnDeclarativeSignatureTemplate {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclarativeSignatures,
    #[return_ref]
    pub parameters: ExplicitParameterDeclarativeSignatureTemplates,
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
    let implicit_parameters = ImplicitParameterDeclarativeSignatures::from_decl(
        decl.implicit_parameters(db),
        declarative_term_region,
        declarative_term_menu,
    );
    let parameters = ExplicitParameterDeclarativeSignatureTemplates::from_decl(
        decl.regular_parameters(db),
        expr_region_data,
        declarative_term_region,
    )?;
    let return_ty = match decl.return_ty(db) {
        Some(return_ty) => declarative_term_region.expr_term(return_ty.expr())?,
        None => declarative_term_menu.unit(),
    };
    Ok(TypeAssociatedFnDeclarativeSignatureTemplate::new(
        db,
        implicit_parameters,
        parameters,
        return_ty,
    ))
}
