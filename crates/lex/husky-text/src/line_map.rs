use crate::*;
use husky_vfs::ModulePath;

#[salsa::tracked(jar = TextJar, return_ref)]
pub(crate) fn module_text_line_map(db: &dyn TextDb, module_path: ModulePath) -> LineMap {
    LineMap::new(module_path.raw_text(db))
}
