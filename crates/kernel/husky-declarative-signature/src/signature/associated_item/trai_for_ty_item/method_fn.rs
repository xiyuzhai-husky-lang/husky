use husky_entity_tree::ImplBlockNode;

use crate::*;

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn trai_for_ty_method_fn_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    decl: TraitForTypeMethodFnDecl,
) -> DeclarativeSignatureResult<TraitForTypeMethodFnDeclarativeSignatureTemplateTemplate> {
    let self_parameter = ExplicitParameterDeclarativeSignature::new(
        match decl.self_parameter(db) {
            Some(self_parameter) => todo!(),
            None => Contract::Pure,
        },
        decl.path(db)
            .impl_block(db)
            .declarative_signature_template(db)?
            .ty(db),
    );
    let expr_region = decl.expr_region(db);
    let expr_region_data = expr_region.data(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    let implicit_parameters = ImplicitParameterDeclarativeSignatures::from_decl(
        decl.implicit_parameters(db),
        declarative_term_region,
        declarative_term_menu,
    );
    let explicit_parameters = ExplicitParameterDeclarativeSignatureTemplates::from_decl(
        decl.explicit_parameters(db),
        expr_region_data,
        declarative_term_region,
    )?;
    let return_ty = match decl.return_ty(db) {
        Some(return_ty) => declarative_term_region.expr_term(return_ty.expr())?,
        None => declarative_term_menu.unit(),
    };
    Ok(
        TraitForTypeMethodFnDeclarativeSignatureTemplateTemplate::new(
            db,
            implicit_parameters,
            self_parameter,
            explicit_parameters,
            return_ty,
        ),
    )
}

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TraitForTypeMethodFnDeclarativeSignatureTemplateTemplate {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclarativeSignatures,
    #[return_ref]
    pub self_parameter: ExplicitParameterDeclarativeSignature,
    #[return_ref]
    pub explicit_parameters: ExplicitParameterDeclarativeSignatureTemplates,
    pub return_ty: DeclarativeTerm,
}
