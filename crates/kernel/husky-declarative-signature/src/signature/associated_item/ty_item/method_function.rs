use crate::*;
use husky_entity_tree::ImplBlockNode;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TypeMethodFunctionDeclarativeSignatureTemplate {
    // todo: formal method, method that is not a function pointer
    #[return_ref]
    pub generic_parameters: DeclarativeGenericParameters,
    #[return_ref]
    pub self_parameter: SpecificRegularParameterDeclarativeSignatureTemplate,
    #[return_ref]
    pub explicit_parameters: DeclarativeSpecificParameters,
    pub return_ty: DeclarativeTerm,
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub fn ty_method_function_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    decl: TypeMethodFnDecl,
) -> DeclarativeSignatureResult<TypeMethodFunctionDeclarativeSignatureTemplate> {
    // todo: overhaul
    // the following is blindly copied from method fn
    let expr_region = decl.expr_region(db);
    let expr_region_data = expr_region.data(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let self_parameter = SpecificRegularParameterDeclarativeSignatureTemplate::new(
        match decl.self_parameter(db) {
            Some(self_parameter) => todo!(),
            None => Contract::None,
        },
        decl.impl_block_path(db)
            .declarative_signature_template(db)?
            .ty(db),
    );
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    let generic_parameters = DeclarativeGenericParameters::from_decl(
        decl.generic_parameters(db),
        declarative_term_region,
        declarative_term_menu,
    );
    let explicit_parameters = DeclarativeSpecificParameters::from_decl(
        decl.explicit_parameters(db),
        expr_region_data,
        declarative_term_region,
    )?;
    let return_ty = match decl.return_ty(db) {
        Some(return_ty) => declarative_term_region.expr_term(return_ty.expr())?,
        None => declarative_term_menu.unit(),
    };
    Ok(TypeMethodFunctionDeclarativeSignatureTemplate::new(
        db,
        generic_parameters,
        self_parameter,
        explicit_parameters,
        return_ty,
    ))
}
