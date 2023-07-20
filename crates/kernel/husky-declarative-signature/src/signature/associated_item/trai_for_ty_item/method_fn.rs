use husky_entity_tree::ImplBlockNode;

use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TraitForTypeMethodFnDeclarativeSignatureTemplate {
    #[return_ref]
    pub generic_parameters: DeclarativeGenericParameterTemplates,
    pub self_parameter: SpecificRegularDeclarativeParameterTemplate,
    /// parenic is a word I coined
    ///
    /// it means things that should be parenthesized
    #[return_ref]
    pub parenic_parameters: DeclarativeParenicParameters,
    pub return_ty: DeclarativeTerm,
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn trai_for_ty_method_fn_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    decl: TraitForTypeMethodFnDecl,
) -> DeclarativeSignatureResult<TraitForTypeMethodFnDeclarativeSignatureTemplate> {
    let self_parameter = SpecificRegularDeclarativeParameterTemplate::new(
        match decl.self_parameter(db) {
            Some(self_parameter) => todo!(),
            None => Contract::None,
        },
        decl.path(db)
            .impl_block(db)
            .declarative_signature_template(db)?
            .self_ty(db)
            .term(),
    );
    let expr_region = decl.expr_region(db);
    let expr_region_data = expr_region.data(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
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
    Ok(TraitForTypeMethodFnDeclarativeSignatureTemplate::new(
        db,
        generic_parameters,
        self_parameter,
        explicit_parameters,
        return_ty,
    ))
}
