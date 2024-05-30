use super::*;

#[salsa::tracked]
pub struct MainCrateDecSignature {
    #[id]
    crate_path: CratePath,
}
