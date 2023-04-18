use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TypeAssociatedFnSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterSignatures,
    #[return_ref]
    pub parameters: ExplicitParameterSignatures,
    pub return_ty: DeclarativeTerm,
}

impl HasDeclarativeSignature for TypeAssociatedFnDecl {
    type DeclarativeSignature = TypeAssociatedFnSignature;

    fn declarative_signature(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<TypeAssociatedFnSignature> {
        ty_associated_fn_signature(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn ty_associated_fn_signature(
    db: &dyn DeclarativeSignatureDb,
    decl: TypeAssociatedFnDecl,
) -> DeclarativeSignatureResult<TypeAssociatedFnSignature> {
    let expr_region = decl.expr_region(db);
    let expr_region_data = expr_region.data(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    let implicit_parameters = ImplicitParameterSignatures::from_decl(
        decl.implicit_parameters(db),
        declarative_term_region,
        declarative_term_menu,
    );
    let parameters = ExplicitParameterSignatures::from_decl(
        decl.parameters(db),
        expr_region_data,
        declarative_term_region,
    )?;
    let return_ty = match decl.return_ty(db) {
        Some(return_ty) => declarative_term_region.expr_term(return_ty.expr())?,
        None => declarative_term_menu.unit(),
    };
    Ok(TypeAssociatedFnSignature::new(
        db,
        implicit_parameters,
        parameters,
        return_ty,
    ))
}
