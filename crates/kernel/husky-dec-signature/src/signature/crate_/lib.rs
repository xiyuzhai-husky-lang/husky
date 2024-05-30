use super::*;

#[salsa::tracked]
pub struct LibCrateDecSignature {
    #[id]
    crate_path: CratePath,
}
