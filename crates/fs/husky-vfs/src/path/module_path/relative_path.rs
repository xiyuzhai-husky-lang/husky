use super::*;
use ::relative_path::RelativePathBuf;

impl ModulePath {
    pub fn relative_dir_for_submodules(self, db: &::salsa::Db) -> &RelativePathBuf {
        module_relative_dir_for_submodules(db, self)
    }

    pub fn relative_stem(self, db: &::salsa::Db) -> &RelativePathBuf {
        module_relative_stem(db, self)
    }
}

#[salsa::tracked(return_ref)]
fn module_relative_dir_for_submodules(
    db: &::salsa::Db,
    module_path: ModulePath,
) -> RelativePathBuf {
    match module_path.data(db) {
        ModulePathData::Root(crate_path) => crate_path.relative_dir_for_submodules(db).clone(),
        ModulePathData::Child { parent, ident } => {
            module_relative_dir_for_submodules(db, parent).join(ident.data(db))
        }
        ModulePathData::Chunk { .. } => unreachable!(),
    }
}

#[salsa::tracked(return_ref)]
fn module_relative_stem(db: &::salsa::Db, module_path: ModulePath) -> RelativePathBuf {
    let dir_for_submodules = module_path.relative_dir_for_submodules(db);
    match module_path.data(db) {
        ModulePathData::Root(crate_path) => match crate_path.kind(db) {
            CrateKind::Lib => dir_for_submodules.join("lib"),
            CrateKind::Main => dir_for_submodules.join("main"),
            CrateKind::Requirements => dir_for_submodules.clone(),
            CrateKind::Task => dir_for_submodules.clone(),
            CrateKind::Bin(_) => todo!(),
            CrateKind::IntegratedTest(_) => todo!(),
            CrateKind::Example => todo!(),
        },
        ModulePathData::Child { .. } => dir_for_submodules.clone(),
        ModulePathData::Chunk { .. } => unreachable!(),
    }
}

#[test]
fn module_relative_dir_for_submodules_works() {
    fn t(module_path: ModulePath, expected: &str, db: &::salsa::Db) {
        assert_eq!(module_path.relative_dir_for_submodules(db), expected);
    }

    let db = &DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let vfs_path_menu = db.vfs_path_menu(toolchain);
    t(vfs_path_menu.core_root(), "src", db);
    t(vfs_path_menu.core_array().inner(), "src/array", db);
}

#[test]
fn module_relative_stem_works() {
    fn t(module_path: ModulePath, expected: &str, db: &::salsa::Db) {
        assert_eq!(module_path.relative_stem(db), expected);
    }

    let db = &DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let vfs_path_menu = db.vfs_path_menu(toolchain);
    t(vfs_path_menu.core_root(), "src/lib", db);
    t(vfs_path_menu.core_array().inner(), "src/array", db);
}
