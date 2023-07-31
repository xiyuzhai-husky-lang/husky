use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct EnumPropsTypeVariantEtherealSignatureTemplate {
    pub parent_ty_template: EnumTypeEtherealSignatureTemplate,
}
