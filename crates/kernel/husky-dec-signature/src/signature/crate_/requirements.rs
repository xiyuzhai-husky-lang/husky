use super::*;

#[salsa::tracked]
pub struct RequirementsCrateDecSignature {
    #[id]
    crate_path: CratePath,
}
