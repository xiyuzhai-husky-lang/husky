use super::*;
use husky_vfs::CratePath;

#[salsa::tracked(jar = LinkageJar)]
fn root_linkage_paths(db: &::salsa::Db, crate_path: CratePath) {}
