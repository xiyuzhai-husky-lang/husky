use super::*;
use husky_syn_decl::decl::crate_::task::TaskCrateSynDecl;

#[salsa::tracked]
pub struct TaskCrateDecSignature {
    #[id]
    crate_path: CratePath,
}

impl TaskCrateDecSignature {
    pub(super) fn from_decl(
        crate_path: CratePath,
        syn_decl: impl Into<Option<TaskCrateSynDecl>>,
        db: &::salsa::Db,
    ) -> DecSignatureResult<Self> {
        Ok(Self::new(db, crate_path))
    }
}
