use super::*;
use husky_dec_signature::signature::assoc_item::trai_item::assoc_static_var::TraitAssocStaticVarDecTemplate;
use husky_entity_path::path::assoc_item::trai_item::TraitItemPath;

#[salsa::interned]
pub struct TraitAssocStaticVarEthTemplate {
    pub path: TraitItemPath,
    pub return_ty: EthTerm,
    pub var_ty: EthTerm,
}

impl TraitAssocStaticVarEthTemplate {
    pub(super) fn from_dec(
        path: TraitItemPath,
        template: TraitAssocStaticVarDecTemplate,
        db: &::salsa::Db,
    ) -> EthSignatureResult<Self> {
        let return_ty = EthTerm::ty_from_dec(db, template.return_ty(db))?;
        let var_ty = EthTerm::ty_from_dec(db, template.var_ty(db))?;
        Ok(Self::new(db, path, return_ty, var_ty))
    }
}

/// # getters
impl TraitAssocStaticVarEthTemplate {
    #[inline(always)]
    pub fn template_parameters(self, db: &::salsa::Db) -> &[EthTemplateParameter] {
        &[]
    }
}
