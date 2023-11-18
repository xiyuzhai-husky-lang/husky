use super::*;
use ::relative_path::{RelativePath, RelativePathBuf};

impl ModulePath {
    pub fn relative_path(self, db: &dyn VfsDb) -> &RelativePath {
        module_relative_path(db, self)
    }
}

#[salsa::tracked(jar = VfsJar, return_ref)]
fn module_relative_path(db: &dyn VfsDb, module_path: ModulePath) -> RelativePathBuf {
    match module_path.data(db) {
        ModulePathData::Root(crate_path) => match crate_path.crate_kind(db) {
            CrateKind::Lib => RelativePathBuf::from_path("lib.rs").unwrap(),
            CrateKind::Main => RelativePathBuf::from_path("main.rs").unwrap(),
            CrateKind::Bin(_) => todo!(),
            CrateKind::IntegratedTest(_) => todo!(),
            CrateKind::Example => todo!(),
        },
        ModulePathData::Child { .. } => module_relative_stem(db, module_path).with_extension("rs"),
    }
}

#[salsa::tracked(jar = VfsJar, return_ref)]
fn module_relative_stem(db: &dyn VfsDb, module_path: ModulePath) -> RelativePathBuf {
    match module_path.data(db) {
        ModulePathData::Root(_) => RelativePathBuf::default(),
        ModulePathData::Child { parent, ident } => {
            module_relative_stem(db, parent).join(ident.data(db))
        }
    }
}

#[test]
fn module_relative_path_works() {
    let mut db = DB::default();
    db.vfs_expect_test_debug_with_db(
        |db, module_path: ModulePath| module_relative_path(db, module_path),
        &VfsTestConfig::new("module_relative_path"),
    )
}
