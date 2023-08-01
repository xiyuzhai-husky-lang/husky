use husky_entity_syn_tree::ImplBlockSynNode;
use husky_syn_expr::SelfParameterDeclPattern;

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
    pub template_parameters: DeclarativeTemplateParameterTemplates,
    pub self_parameter: DeclarativeTermRitchieRegularParameter,
    #[return_ref]
    pub parenate_parameters: DeclarativeParenateParameters,
    pub return_ty: DeclarativeTerm,
}

impl HasDeclarativeSignatureTemplate for TypeMethodFnSynDecl {
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
    decl: TypeMethodFnSynDecl,
) -> DeclarativeSignatureResult<TypeMethodFnDeclarativeSignatureTemplate> {
    let syn_expr_region = decl.syn_expr_region(db);
    let expr_region_data = syn_expr_region.data(db);
    let declarative_term_region = declarative_term_region(db, syn_expr_region);
    let impl_block = decl
        .impl_block_path(db)
        .declarative_signature_template(db)?;
    let self_ty = impl_block.ty(db);
    let contract = match decl.self_parameter(db) {
        Some(self_parameter) => Contract::new(self_parameter.ephem_symbol_modifier_token_group()),
        None => Contract::None,
    };
    let self_parameter = DeclarativeTermRitchieRegularParameter::new(contract, self_ty);
    let declarative_term_menu = db
        .declarative_term_menu(syn_expr_region.toolchain(db))
        .unwrap();
    let template_parameters = DeclarativeTemplateParameterTemplates::from_decl(
        decl.template_parameters(db),
        declarative_term_region,
        declarative_term_menu,
    );
    let parenate_parameters = DeclarativeParenateParameters::from_decl(
        decl.parenate_parameters(db),
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
        template_parameters,
        self_parameter,
        parenate_parameters,
        return_ty,
    ))
}
