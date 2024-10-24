use super::*;
use husky_entity_path::path::assoc_item::ty_item::TypeItemPath;
use husky_syn_decl::decl::assoc_item::ty_item::assoc_ritchie::TypeAssocRitchieSynDecl;

#[salsa::interned]
pub struct TypeAssocRitchieDecTemplate {
    pub path: TypeItemPath,
    /// the term for `Self`
    /// not necessarily equal to the type of `self`
    ///
    /// we don't use self_ty_arguments because it's not determined for declarative terms
    pub self_ty: DecTerm,
    #[return_ref]
    pub template_parameters: DecTemplateParameters,
    #[return_ref]
    pub parenate_parameters: DeclarativeParenateParameters,
    pub return_ty: DecTerm,
}

impl TypeAssocRitchieDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        path: TypeItemPath,
        decl: TypeAssocRitchieSynDecl,
    ) -> DecSignatureResult<Self> {
        let syn_expr_region = decl.syn_expr_region(db);
        let expr_region_data = syn_expr_region.data(db);
        let dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let dec_term_menu = db.dec_term_menu(syn_expr_region.toolchain(db)).unwrap();
        let impl_block = decl.path(db).impl_block(db).dec_template(db)?;
        let self_ty = impl_block.ty(db);
        let template_parameters = DecTemplateParameters::from_decl(
            decl.template_parameters(db),
            dec_term_region,
            dec_term_menu,
        );
        let parenate_parameters = DeclarativeParenateParameters::from_decl(
            decl.parenate_parameters(db),
            expr_region_data,
            dec_term_region,
        )?;
        let return_ty = match decl.return_ty(db) {
            Some(return_ty) => dec_term_region.expr_term(return_ty.syn_expr_idx())?,
            None => dec_term_menu.unit(),
        };
        Ok(TypeAssocRitchieDecTemplate::new(
            db,
            path,
            self_ty,
            template_parameters,
            parenate_parameters,
            return_ty,
        ))
    }
}
