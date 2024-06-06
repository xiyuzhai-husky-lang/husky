use super::*;
use ::relative_path::RelativePathBuf;

impl CratePath {
    pub fn relative_dir_for_submodules(self, db: &::salsa::Db) -> &RelativePathBuf {
        crate_relative_dir_for_submodules(db, self)
    }
}

#[salsa::tracked(return_ref)]
fn crate_relative_dir_for_submodules(db: &::salsa::Db, crate_path: CratePath) -> RelativePathBuf {
    match crate_path.kind(db) {
        CrateKind::Lib => "src".into(),
        CrateKind::Main => "src".into(),
        CrateKind::Requirements => "requirements".into(),
        CrateKind::Task => "task".into(),
        CrateKind::Bin(_) => todo!(),
        CrateKind::IntegratedTest(_) => todo!(),
        CrateKind::Example => todo!(),
    }
}
