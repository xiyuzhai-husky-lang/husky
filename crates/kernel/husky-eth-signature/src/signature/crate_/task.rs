use super::*;
use husky_dec_signature::signature::crate_::task::TaskCrateDecSignature;

#[salsa::tracked]
pub struct TaskCrateEthSignature {
    #[id]
    crate_path: CratePath,
}

impl TaskCrateEthSignature {
    pub(super) fn from_dec(
        crate_path: CratePath,
        dec_signature: TaskCrateDecSignature,
        db: &::salsa::Db,
    ) -> EthSignatureResult<Self> {
        Ok(Self::new(db, crate_path))
    }
}
