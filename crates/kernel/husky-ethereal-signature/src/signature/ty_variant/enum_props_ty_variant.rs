use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct EnumPropsVariantEthTemplate {
    pub parent_ty_template: EnumTypeEthTemplate,
    pub instance_constructor_ritchie_ty: EtherealTerm,
}

impl EnumPropsVariantEthTemplate {
    pub fn instance_constructor_ty(self, db: &::salsa::Db) -> EtherealTerm {
        self.instance_constructor_ritchie_ty(db).into()
    }
}
