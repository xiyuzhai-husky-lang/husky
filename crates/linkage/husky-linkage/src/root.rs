use super::*;
use husky_vfs::CratePath;

#[salsa::tracked(jar = LinkageJar)]
fn root_linkage_paths(db: &dyn LinkageDb, crate_path: CratePath) {}
