use super::*;
use crate::signature::major_item::ty::r#enum::EnumEthTemplate;

#[salsa::interned]
pub struct EnumPropsVariantEthTemplate {
    pub path: TypeVariantPath,
    pub parent_ty_template: EnumEthTemplate,
    pub instance_constructor_ritchie_ty: EthTerm,
}

impl EnumPropsVariantEthTemplate {
    pub fn instance_constructor_ty(self, db: &::salsa::Db) -> EthTerm {
        self.instance_constructor_ritchie_ty(db).into()
    }
}
