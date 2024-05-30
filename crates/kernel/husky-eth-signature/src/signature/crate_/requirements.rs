use super::*;
use husky_dec_signature::signature::crate_::requirements::RequirementsCrateDecSignature;

#[salsa::tracked]
pub struct RequirementsCrateEthSignature {
    #[id]
    crate_path: CratePath,
}

impl RequirementsCrateEthSignature {
    pub(super) fn from_dec(
        crate_path: CratePath,
        dec_signature: RequirementsCrateDecSignature,
        db: &::salsa::Db,
    ) -> EthSignatureResult<Self> {
        Ok(Self::new(db, crate_path))
    }
}
