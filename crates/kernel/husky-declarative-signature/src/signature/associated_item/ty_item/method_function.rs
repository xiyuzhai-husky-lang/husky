use crate::*;
use husky_entity_syn_tree::ImplBlockSynNode;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TypeMethodFunctionDeclarativeSignatureTemplate {
    // todo: formal method, method that is not a function pointer
    #[return_ref]
    pub generic_parameters: DeclarativeGenericParameterTemplates,
    pub self_parameter: DeclarativeTermRitchieRegularParameter,
    #[return_ref]
    pub parenic_parameters: DeclarativeParenicParameters,
    pub return_ty: DeclarativeTerm,
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub fn ty_method_function_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    decl: TypeMethodFnSynDecl,
) -> DeclarativeSignatureResult<TypeMethodFunctionDeclarativeSignatureTemplate> {
    // todo: overhaul
    // the following is blindly copied from method fn
    let syn_expr_region = decl.syn_expr_region(db);
    let expr_region_data = syn_expr_region.data(db);
    let declarative_term_region = declarative_term_region(db, syn_expr_region);
    let self_parameter = DeclarativeTermRitchieRegularParameter::new(
        match decl.self_parameter(db) {
            Some(self_parameter) => todo!(),
            None => Contract::None,
        },
        decl.impl_block_path(db)
            .declarative_signature_template(db)?
            .ty(db),
    );
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
    Ok(TypeMethodFunctionDeclarativeSignatureTemplate::new(
        db,
        generic_parameters,
        self_parameter,
        parenic_parameters,
        return_ty,
    ))
}
