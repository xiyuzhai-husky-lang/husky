use super::*;
use husky_syn_decl::decl::crate_::main::MainCrateSynDecl;

#[salsa::tracked]
pub struct MainCrateDecSignature {
    #[id]
    crate_path: CratePath,
}

impl MainCrateDecSignature {
    pub(super) fn from_decl(
        crate_path: CratePath,
        syn_decl: impl Into<Option<MainCrateSynDecl>>,
        db: &::salsa::Db,
    ) -> DecSignatureResult<Self> {
        Ok(Self::new(db, crate_path))
    }
}
