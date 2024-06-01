use super::*;
use ::relative_path::RelativePathBuf;

#[salsa::tracked(return_ref)]
pub fn module_relative_stem(db: &::salsa::Db, module_path: ModulePath) -> RelativePathBuf {
    match module_path.data(db) {
        ModulePathData::Root(_) => RelativePathBuf::default(),
        ModulePathData::Child { parent, ident } => {
            module_relative_stem(db, parent).join(ident.data(db))
        }
        ModulePathData::Script { .. } => unreachable!(),
    }
}
