use super::*;
use husky_dec_signature::signature::major_item::form::static_var::MajorStaticVarDecTemplate;

#[salsa::interned]
pub struct MajorStaticVarEthTemplate {
    pub path: MajorFormPath,
    pub return_ty: EthTerm,
}

impl MajorStaticVarEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: MajorFormPath,
        dec_template: MajorStaticVarDecTemplate,
    ) -> EthSignatureResult<Self> {
        let return_ty = EthTerm::ty_from_dec(db, dec_template.return_ty(db))?;
        Ok(Self::new(db, path, return_ty))
    }
}
