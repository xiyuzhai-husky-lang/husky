use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct EnumUnitVariantDeclarativeSignatureTemplate {
    pub parent_ty_template: EnumDeclarativeSignatureTemplate,
    pub ty: DeclarativeTerm,
}
