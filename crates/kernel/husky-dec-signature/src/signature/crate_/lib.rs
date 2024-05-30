use super::*;
use husky_syn_decl::decl::crate_::lib::LibCrateSynDecl;

#[salsa::tracked]
pub struct LibCrateDecSignature {
    #[id]
    crate_path: CratePath,
}

impl LibCrateDecSignature {
    pub(super) fn from_decl(
        crate_path: CratePath,
        syn_decl: impl Into<Option<LibCrateSynDecl>>,
        db: &::salsa::Db,
    ) -> DecSignatureResult<Self> {
        Ok(Self::new(db, crate_path))
    }
}
