use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct VfsPathMenu {
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
    core_option: ModulePath,
    core_slice: ModulePath,
    /// core::num
    core_num: ModulePath,
}

#[salsa::tracked(jar = VfsJar, return_ref)]
pub(crate) fn vfs_path_menu(db: &dyn VfsDb, toolchain: Toolchain) -> ToolchainResult<VfsPathMenu> {
    VfsPathMenu::new(db, toolchain)
}

impl VfsPathMenu {
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
        let core_option = ModulePath::new_child(db, core, db.it_ident_borrowed("option").unwrap());
        let core_slice = ModulePath::new_child(db, core, db.it_ident_borrowed("slice").unwrap());
        Ok(Self {
            core_package,
            std_package,
            core_library,
            std_library,
            core,
            std,
            core_prelude,
            core_ops,
            core_option,
            core_slice,
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

    pub fn core_option(&self) -> ModulePath {
        self.core_option
    }

    pub fn core_slice(&self) -> ModulePath {
        self.core_slice
    }
}
