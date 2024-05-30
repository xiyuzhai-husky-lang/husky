use husky_dec_signature::signature::HasDecTemplate;
use husky_entity_path::path::impl_block::ty_impl_block::TypeImplBlockPath;

use super::*;

#[salsa::interned]
pub struct TypeImplBlockEthTemplate {
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
    pub self_ty: EthTerm,
}

impl HasEthTemplate for TypeImplBlockPath {
    type EthTemplate = TypeImplBlockEthTemplate;

    fn eth_template(self, db: &::salsa::Db) -> EthSignatureResult<Self::EthTemplate> {
        ty_impl_block_eth_template(db, self)
    }
}

#[salsa::tracked]
pub(crate) fn ty_impl_block_eth_template(
    db: &::salsa::Db,
    path: TypeImplBlockPath,
) -> EthSignatureResult<TypeImplBlockEthTemplate> {
    let dec_template = path.dec_template(db)?;
    let template_parameters =
        EthTemplateParameters::from_dec(db, dec_template.template_parameters(db))?;
    let ty = EthTerm::ty_from_dec(db, dec_template.ty(db))?;
    Ok(TypeImplBlockEthTemplate::new(db, template_parameters, ty))
}
