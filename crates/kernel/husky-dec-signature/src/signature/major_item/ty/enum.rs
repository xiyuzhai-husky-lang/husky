use super::*;
use husky_syn_decl::decl::major_item::ty::r#enum::EnumSynDecl;

#[salsa::interned]
pub struct EnumDecTemplate {
    #[return_ref]
    pub template_parameters: DecTemplateParameters,
    pub self_ty: DecTerm,
}

impl EnumDecTemplate {
    pub fn from_decl(
        db: &::salsa::Db,
        path: TypePath,
        decl: EnumSynDecl,
    ) -> DecSignatureResult<Self> {
        let syn_expr_region = decl.syn_expr_region(db);
        let dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let dec_term_menu = db.dec_term_menu(syn_expr_region.toolchain(db)).unwrap();
        let template_parameters = DecTemplateParameters::from_decl(
            decl.template_parameters(db),
            &dec_term_region,
            dec_term_menu,
        );
        let self_ty = construct_self_ty(db, path, &template_parameters);
        Ok(Self::new(db, template_parameters, self_ty))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
pub struct EnumDecSignature {}
