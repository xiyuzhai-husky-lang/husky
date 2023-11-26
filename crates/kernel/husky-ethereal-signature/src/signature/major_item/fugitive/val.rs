use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct ValFugitiveEtherealSignatureTemplate {
    pub path: FugitivePath,
}

impl ValFugitiveEtherealSignatureTemplate {
    pub(super) fn from_declarative(
        db: &::salsa::Db,
        path: FugitivePath,
        _declarative_signature_template: ValFugitiveDeclarativeSignatureTemplate,
    ) -> EtherealSignatureResult<Self> {
        Ok(Self::new(db, path))
    }
}
