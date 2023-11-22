use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct EnumPropsVariantEtherealSignatureTemplate {
    pub parent_ty_template: EnumTypeEtherealSignatureTemplate,
}
