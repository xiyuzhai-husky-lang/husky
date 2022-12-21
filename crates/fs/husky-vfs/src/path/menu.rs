use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct PathMenu {}

#[salsa::tracked(jar = VfsJar, return_ref)]
pub(crate) fn path_menu(db: &dyn VfsDb, toolchain: Toolchain) -> VfsResult<PathMenu> {
    todo!()
}

impl PathMenu {
    pub fn core_package(&self) -> PackagePath {
        todo!()
    }

    pub fn std_package(&self) -> PackagePath {
        todo!()
    }

    pub fn core_library(&self) -> CratePath {
        todo!()
    }

    pub fn core(&self) -> ModulePath {
        todo!()
    }

    pub fn core_basic(&self) -> ModulePath {
        todo!()
    }

    pub fn core_num(&self) -> ModulePath {
        todo!()
    }

    pub fn core_prelude(&self) -> ModulePath {
        todo!()
    }

    pub fn std(&self) -> ModulePath {
        todo!()
    }
}
