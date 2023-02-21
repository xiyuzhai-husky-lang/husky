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
    core_basic: ModulePath,
    core_mem: ModulePath,
    /// core::num
    core_num: ModulePath,
    core_slice: ModulePath,
    core_str: ModulePath,
    core_ops: ModulePath,
    core_option: ModulePath,
    core_prelude: ModulePath,
    core_raw_bits: ModulePath,
    core_vec: ModulePath,
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
        let core_basic = ModulePath::new_child(db, core, db.it_ident_borrowed("basic").unwrap());
        let core_mem = ModulePath::new_child(db, core, db.it_ident_borrowed("mem").unwrap());
        let core_num = ModulePath::new_child(db, core, db.it_ident_borrowed("num").unwrap());
        let core_ops = ModulePath::new_child(db, core, db.it_ident_borrowed("ops").unwrap());
        let core_option = ModulePath::new_child(db, core, db.it_ident_borrowed("option").unwrap());
        let core_prelude =
            ModulePath::new_child(db, core, db.it_ident_borrowed("prelude").unwrap());
        let core_raw_bits =
            ModulePath::new_child(db, core, db.it_ident_borrowed("raw_bits").unwrap());
        let core_slice = ModulePath::new_child(db, core, db.it_ident_borrowed("slice").unwrap());
        let core_str = ModulePath::new_child(db, core, db.it_ident_borrowed("str").unwrap());
        let core_vec = ModulePath::new_child(db, core, db.it_ident_borrowed("vec").unwrap());
        Ok(Self {
            core_package,
            std_package,
            core_library,
            std_library,
            core,
            std,
            core_basic,
            core_mem,
            core_num,
            core_ops,
            core_option,
            core_prelude,
            core_raw_bits,
            core_slice,
            core_str,
            core_vec,
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

    /// core::mem
    pub fn core_mem(&self) -> ModulePath {
        self.core_mem
    }

    /// core::num
    pub fn core_num(&self) -> ModulePath {
        self.core_num
    }

    pub fn core_prelude(&self) -> ModulePath {
        self.core_prelude
    }

    /// core::raw_bits
    pub fn core_raw_bits(&self) -> ModulePath {
        self.core_raw_bits
    }

    /// core::vec
    pub fn core_vec(&self) -> ModulePath {
        self.core_vec
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

    pub fn core_str(&self) -> ModulePath {
        self.core_str
    }
}

#[test]
fn vfs_path_menu_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let menu = db.vfs_path_menu(toolchain).unwrap();
    assert_eq!(menu.core().to_string_with_db(&db), "core");
    assert_eq!(menu.std().to_string_with_db(&db), "std");
    assert_eq!(menu.core_basic().to_string_with_db(&db), "core::basic");
    assert_eq!(menu.core_mem().to_string_with_db(&db), "core::mem");
    assert_eq!(menu.core_num().to_string_with_db(&db), "core::num");
    assert_eq!(menu.core_slice().to_string_with_db(&db), "core::slice");
    assert_eq!(menu.core_str().to_string_with_db(&db), "core::str");
    assert_eq!(menu.core_ops().to_string_with_db(&db), "core::ops");
    assert_eq!(menu.core_option().to_string_with_db(&db), "core::option");
    assert_eq!(menu.core_prelude().to_string_with_db(&db), "core::prelude");
    assert_eq!(
        menu.core_raw_bits().to_string_with_db(&db),
        "core::raw_bits"
    );
    assert_eq!(menu.core_vec().to_string_with_db(&db), "core::vec");
}
