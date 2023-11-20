use super::*;
use husky_vfs::CratePath;

#[salsa::tracked(jar = LinkagePathJar)]
fn root_linkage_paths(db: &dyn LinkagePathDb, crate_path: CratePath) {}
