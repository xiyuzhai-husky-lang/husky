use super::*;
use husky_entity_path::path::impl_block::ty_impl_block::TypeImplBlockPath;

#[salsa::interned]
pub struct TypeImplBlockDecTemplate {
    #[return_ref]
    pub template_parameters: DecTemplateParameters,
    pub ty: DecTerm,
}

impl HasDecTemplate for TypeImplBlockPath {
    type DecTemplate = TypeImplBlockDecTemplate;

    fn dec_template(self, db: &::salsa::Db) -> DecSignatureResult<Self::DecTemplate> {
        ty_impl_block_syn_dec_template(db, self)
    }
}

#[salsa::tracked]
pub(crate) fn ty_impl_block_syn_dec_template(
    db: &::salsa::Db,
    path: TypeImplBlockPath,
) -> DecSignatureResult<TypeImplBlockDecTemplate> {
    let decl = path.syn_decl(db)?;
    let syn_expr_region = decl.syn_expr_region(db);
    let dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
    let dec_term_menu = db.dec_term_menu(syn_expr_region.toolchain(db)).unwrap();
    let template_parameters = DecTemplateParameters::from_decl(
        decl.template_parameters(db),
        &dec_term_region,
        dec_term_menu,
    );
    let self_ty_expr = decl.self_ty_expr(db);
    let ty = dec_term_region.expr_term(self_ty_expr.expr())?;
    Ok(TypeImplBlockDecTemplate::new(db, template_parameters, ty))
}
