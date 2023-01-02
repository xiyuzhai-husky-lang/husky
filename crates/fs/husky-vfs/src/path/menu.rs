use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct PathMenu {
    core_package: PackagePath,
    std_package: PackagePath,
    core_library: CratePath,
    std_library: CratePath,
    /// core
    core: ModulePath,
    std: ModulePath,
    core_prelude: ModulePath,
    core_basic: ModulePath,
    core_ops: ModulePath,
    /// core::num
    core_num: ModulePath,
}

#[salsa::tracked(jar = VfsJar, return_ref)]
pub(crate) fn path_menu(db: &dyn VfsDb, toolchain: Toolchain) -> ToolchainResult<PathMenu> {
    PathMenu::new(db, toolchain)
}

impl PathMenu {
    fn new(db: &dyn VfsDb, toolchain: Toolchain) -> ToolchainResult<Self> {
        let word_menu = db.word_menu();
        let core_package = PackagePath::new_toolchain_package(db, toolchain, word_menu.core())?;
        let std_package = PackagePath::new_toolchain_package(db, toolchain, word_menu.std())?;
        let core_library = CratePath::new(db, core_package, CrateKind::Library);
        let std_library = CratePath::new(db, std_package, CrateKind::Library);
        let core = ModulePath::new_root(db, core_library);
        let std = ModulePath::new_root(db, std_library);
        let core_prelude =
            ModulePath::new_child(db, core, db.it_ident_borrowed("prelude").unwrap());
        let core_basic = ModulePath::new_child(db, core, db.it_ident_borrowed("basic").unwrap());
        let core_num = ModulePath::new_child(db, core, db.it_ident_borrowed("num").unwrap());
        let core_ops = ModulePath::new_child(db, core, db.it_ident_borrowed("ops").unwrap());
        Ok(Self {
            core_package,
            std_package,
            core_library,
            std_library,
            core,
            std,
            core_prelude,
            core_ops,
            core_num,
            core_basic,
        })
    }

    pub fn core_package(&self) -> PackagePath {
        self.core_package
    }

    pub fn std_package(&self) -> PackagePath {
        self.std_package
    }

    pub fn core_library(&self) -> CratePath {
        self.core_library
    }

    pub fn core(&self) -> ModulePath {
        self.core
    }

    pub fn core_basic(&self) -> ModulePath {
        self.core_basic
    }

    /// core::num
    pub fn core_num(&self) -> ModulePath {
        self.core_num
    }

    pub fn core_prelude(&self) -> ModulePath {
        self.core_prelude
    }

    pub fn std(&self) -> ModulePath {
        self.std
    }

    pub fn core_ops(&self) -> ModulePath {
        self.core_ops
    }
}
