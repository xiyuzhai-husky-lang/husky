use super::*;
use husky_dec_signature::signature::crate_::main::MainCrateDecSignature;

#[salsa::interned]
pub struct MainCrateEthSignature {
    pub crate_path: CratePath,
}

impl MainCrateEthSignature {
    pub(super) fn from_dec(
        crate_path: CratePath,
        dec_signature: MainCrateDecSignature,
        db: &::salsa::Db,
    ) -> EthSignatureResult<Self> {
        Ok(Self::new(db, crate_path))
    }
}
