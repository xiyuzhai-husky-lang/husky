use super::*;

#[salsa::interned(db = DecSignatureDb, jar = DecSignatureJar)]
pub struct TypeImplBlockDecTemplate {
    #[return_ref]
    pub template_parameters: DeclarativeTemplateParameterTemplates,
    pub ty: DeclarativeTerm,
}

impl HasDecTemplate for TypeImplBlockPath {
    type DecTemplate = TypeImplBlockDecTemplate;

    fn dec_template(self, db: &::salsa::Db) -> DecSignatureResult<Self::DecTemplate> {
        ty_impl_block_syn_dec_template(db, self)
    }
}

#[salsa::tracked(jar = DecSignatureJar)]
pub(crate) fn ty_impl_block_syn_dec_template(
    db: &::salsa::Db,
    path: TypeImplBlockPath,
) -> DecSignatureResult<TypeImplBlockDecTemplate> {
    let decl = path.syn_decl(db)?;
    let syn_expr_region = decl.syn_expr_region(db);
    let declarative_term_region = declarative_term_region(db, syn_expr_region);
    let declarative_term_menu = db
        .declarative_term_menu(syn_expr_region.toolchain(db))
        .unwrap();
    let template_parameters = DeclarativeTemplateParameterTemplates::from_decl(
        decl.template_parameters(db),
        &declarative_term_region,
        declarative_term_menu,
    );
    let self_ty_expr = decl.self_ty_expr(db);
    let ty = declarative_term_region.expr_term(self_ty_expr.expr())?;
    Ok(TypeImplBlockDecTemplate::new(db, template_parameters, ty))
}
