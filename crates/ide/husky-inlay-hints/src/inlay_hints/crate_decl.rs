use super::*;
use husky_entity_path::path::ItemPath;
use husky_vfs::path::crate_path::CratePath;

#[salsa::tracked(return_ref)]
pub(crate) fn crate_decl_inlay_hints(db: &::salsa::Db, crate_path: CratePath) -> Vec<InlayHint> {
    vec![]
}
