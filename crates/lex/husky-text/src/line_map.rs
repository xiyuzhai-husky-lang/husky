use crate::*;
use husky_vfs::ModulePath;

#[salsa::tracked(jar = TextJar, return_ref)]
pub(crate) fn module_text_line_map(db: &::salsa::Db, module_path: ModulePath) -> LineMap {
    LineMap::new(module_path.raw_text(db))
}
