use super::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct EnumPropsVariantDeclarativeSignatureTemplate {
    pub parent_ty_template: EnumTypeDeclarativeSignatureTemplate,
}
