use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct EnumPropsVariantEtherealSignatureTemplate {
    pub parent_ty_template: EnumTypeEtherealSignatureTemplate,
    pub instance_constructor_ritchie_ty: EtherealTerm,
}

impl EnumPropsVariantEtherealSignatureTemplate {
    pub fn instance_constructor_ty(self, db: &::salsa::Db) -> EtherealTerm {
        self.instance_constructor_ritchie_ty(db).into()
    }
}
