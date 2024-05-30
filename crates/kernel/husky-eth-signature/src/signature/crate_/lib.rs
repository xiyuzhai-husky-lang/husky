use super::*;
use husky_dec_signature::signature::crate_::lib::LibCrateDecSignature;

#[salsa::interned]
pub struct LibCrateEthSignature {
    pub crate_path: CratePath,
}

impl LibCrateEthSignature {
    pub(super) fn from_dec(
        crate_path: CratePath,
        dec_signature: LibCrateDecSignature,
        db: &::salsa::Db,
    ) -> EthSignatureResult<Self> {
        Ok(Self::new(db, crate_path))
    }
}
