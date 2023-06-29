use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct UnitVariantDeclarativeSignatureTemplate {
    pub ty: DeclarativeTerm,
}
