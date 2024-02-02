use super::*;

#[salsa::interned(db = EtherealSignatureDb, jar = EtherealSignatureJar)]
pub struct ValFugitiveEthTemplate {
    pub path: FugitivePath,
    pub return_ty: EthTerm,
}

impl ValFugitiveEthTemplate {
    pub(super) fn from_declarative(
        db: &::salsa::Db,
        path: FugitivePath,
        dec_template: MajorValDecTemplate,
    ) -> EtherealSignatureResult<Self> {
        let return_ty = EthTerm::ty_from_declarative(db, dec_template.return_ty(db))?;
        Ok(Self::new(db, path, return_ty))
    }
}
