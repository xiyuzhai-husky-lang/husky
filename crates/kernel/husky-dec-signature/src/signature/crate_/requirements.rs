use super::*;
use husky_syn_decl::decl::crate_::requirements::RequirementsCrateSynDecl;

#[salsa::tracked]
pub struct RequirementsCrateDecSignature {
    #[id]
    crate_path: CratePath,
}

impl RequirementsCrateDecSignature {
    pub(super) fn from_decl(
        crate_path: CratePath,
        syn_decl: impl Into<Option<RequirementsCrateSynDecl>>,
        db: &::salsa::Db,
    ) -> DecSignatureResult<Self> {
        Ok(Self::new(db, crate_path))
    }
}
