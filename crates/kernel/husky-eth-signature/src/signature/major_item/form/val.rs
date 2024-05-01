use super::*;

#[salsa::interned]
pub struct MajorValEthTemplate {
    pub path: MajorFormPath,
    pub return_ty: EthTerm,
}

impl MajorValEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: MajorFormPath,
        dec_template: MajorValDecTemplate,
    ) -> EtherealSignatureResult<Self> {
        let return_ty = EthTerm::ty_from_dec(db, dec_template.return_ty(db))?;
        Ok(Self::new(db, path, return_ty))
    }
}
