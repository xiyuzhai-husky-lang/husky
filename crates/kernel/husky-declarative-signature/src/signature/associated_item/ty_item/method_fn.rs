use husky_entity_tree::ImplBlockNode;
use husky_expr::SelfParameterDeclPattern;

use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TypeMethodFnDeclarativeSignatureTemplate {
    pub impl_block: TypeImplBlockDeclarativeSignatureTemplate,
    /// the term for `Self`
    /// not necessarily equal to the type of `self`
    ///
    /// we don't use self_ty_arguments because it's not determined for declarative terms
    pub self_ty: DeclarativeTerm,
    // todo: formal method, method that is not a function pointer
    #[return_ref]
    pub generic_parameters: DeclarativeGenericParameterTemplates,
    pub self_parameter: DeclarativeTermRitchieRegularParameter,
    #[return_ref]
    pub parenic_parameters: DeclarativeParenicParameters,
    pub return_ty: DeclarativeTerm,
}

impl HasDeclarativeSignatureTemplate for TypeMethodFnDecl {
    type DeclarativeSignatureTemplate = TypeMethodFnDeclarativeSignatureTemplate;

    fn declarative_signature_template(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignatureTemplate> {
        ty_method_fn_declarative_signature_template(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub fn ty_method_fn_declarative_signature_template(
    db: &dyn DeclarativeSignatureDb,
    decl: TypeMethodFnDecl,
) -> DeclarativeSignatureResult<TypeMethodFnDeclarativeSignatureTemplate> {
    let expr_region = decl.expr_region(db);
    let expr_region_data = expr_region.data(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let impl_block = decl
        .impl_block_path(db)
        .declarative_signature_template(db)?;
    let self_ty = impl_block.ty(db);
    let contract = match decl.self_parameter(db) {
        Some(self_parameter) => match self_parameter {
            SelfParameterDeclPattern::Pure { .. } => Contract::None,
            SelfParameterDeclPattern::Owned { .. } => todo!(),
            SelfParameterDeclPattern::Mut { .. } => Contract::BorrowMut,
            SelfParameterDeclPattern::MutOwned { .. } => todo!(),
        },
        None => Contract::None,
    };
    let self_parameter = DeclarativeTermRitchieRegularParameter::new(contract, self_ty);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
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
    Ok(TypeMethodFnDeclarativeSignatureTemplate::new(
        db,
        impl_block,
        self_ty,
        generic_parameters,
        self_parameter,
        parenic_parameters,
        return_ty,
    ))
}
