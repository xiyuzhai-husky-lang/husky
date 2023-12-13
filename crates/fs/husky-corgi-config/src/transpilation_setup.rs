use husky_coword::{Ident, Kebab};
use husky_vfs::linktime_target_path::{LinktimeTargetPath, LinktimeTargetPathData};

use crate::*;

#[salsa::interned(jar = CorgiConfigJar)]
pub struct TranspilationSetup {
    #[return_ref]
    _rust_data: Option<RustTranspilationSetupData>,
    #[return_ref]
    _mlir_data: Option<MlirTranspilationSetupData>,
}

pub trait HasTranspilationSetup: Copy {
    fn transpilation_setup(self, db: &::salsa::Db) -> TranspilationSetup;
}

impl HasTranspilationSetup for LinktimeTargetPath {
    fn transpilation_setup(self, db: &::salsa::Db) -> TranspilationSetup {
        linktime_target_transpilation_setup(db, self)
    }
}

#[salsa::tracked(jar = CorgiConfigJar)]
pub fn linktime_target_transpilation_setup(
    db: &::salsa::Db,
    path: LinktimeTargetPath,
) -> TranspilationSetup {
    match path.data(db) {
        LinktimeTargetPathData::Package(package_path) => TranspilationSetup::new(
            db,
            Some(RustTranspilationSetupData::new_ad_hoc(package_path, db)),
            None,
        ),
        LinktimeTargetPathData::Workspace(_) => todo!(),
    }
}

impl TranspilationSetup {
    #[cfg(debug_assertions)]
    pub fn new_ad_hoc(package_path: PackagePath, db: &::salsa::Db) -> Self {
        Self::new(
            db,
            Some(RustTranspilationSetupData::new_ad_hoc(package_path, db)),
            None,
        )
    }

    pub fn rust_data<'a>(self, db: &'a ::salsa::Db) -> Option<&'a RustTranspilationSetupData> {
        self._rust_data(db).as_ref()
    }

    pub fn c_data<'a>(self, db: &'a ::salsa::Db) -> Option<&'a CTranspilationSetupData> {
        todo!()
    }

    pub fn llvm_data<'a>(self, db: &'a ::salsa::Db) -> Option<&'a LlvmTranspilationSetupData> {
        todo!()
    }

    pub fn mlir_data<'a>(self, db: &'a ::salsa::Db) -> Option<&'a MlirTranspilationSetupData> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RustTranspilationSetupData {
    pub task_dependency_name: Kebab,
    pub task_dependency_ident: Ident,
    pub task_dependency_path: VirtualPath,
}

impl RustTranspilationSetupData {
    fn new_ad_hoc(package_path: PackagePath, db: &::salsa::Db) -> Self {
        Self {
            task_dependency_name: Kebab::from_ref(db, "ad-hoc-task-dependency").unwrap(),
            task_dependency_ident: Ident::from_ref(db, "ad_hoc_task_dependency").unwrap(),
            task_dependency_path: package_path
                .registry_path(db)
                .unwrap()
                .path()
                .join("ad-hoc-task-dependency", db),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CTranspilationSetupData {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LlvmTranspilationSetupData {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MlirTranspilationSetupData {}
