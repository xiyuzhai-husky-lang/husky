use super::*;

#[salsa::tracked]
pub struct TaskCrateDecSignature {
    #[id]
    crate_path: CratePath,
}
