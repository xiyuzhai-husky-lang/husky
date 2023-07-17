use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct VfsPathMenu {
    core_package: PackagePath,
    std_package: PackagePath,
    core_library: CratePath,
    std_library: CratePath,
    /// core
    core_root: ModulePath,
    std: ModulePath,
    core_array: ModulePath,
    core_basic: ModulePath,
    core_clone: ModulePath,
    core_default: ModulePath,
    core_list: ModulePath,
    core_marker: ModulePath,
    core_mem: ModulePath,
    /// core::num
    core_num: ModulePath,
    core_slice: ModulePath,
    core_str: ModulePath,
    core_ops: ModulePath,
    core_option: ModulePath,
    core_prelude: ModulePath,
    core_raw_bits: ModulePath,
    core_result: ModulePath,
    core_visual: ModulePath,
}

#[salsa::tracked(jar = VfsJar, return_ref)]
pub(crate) fn vfs_path_menu(db: &dyn VfsDb, toolchain: Toolchain) -> VfsPathMenu {
    VfsPathMenu::new(db, toolchain)
}

impl VfsPathMenu {
    fn new(db: &dyn VfsDb, toolchain: Toolchain) -> Self {
        let coword_menu = db.coword_menu();
        let core_package =
            PackagePath::new_toolchain_package(db, toolchain, coword_menu.core_name());
        let std_package = PackagePath::new_toolchain_package(db, toolchain, coword_menu.std_name());
        let core_library = CratePath::new(db, core_package, CrateKind::Library);
        let std_library = CratePath::new(db, std_package, CrateKind::Library);
        let core = ModulePath::new_root(db, core_library);
        let std = ModulePath::new_root(db, std_library);
        let core_array = ModulePath::new_child(db, core, db.it_ident_borrowed("array").unwrap());
        let core_basic = ModulePath::new_child(db, core, db.it_ident_borrowed("basic").unwrap());
        let core_clone = ModulePath::new_child(db, core, db.it_ident_borrowed("clone").unwrap());
        let core_default =
            ModulePath::new_child(db, core, db.it_ident_borrowed("default").unwrap());
        let core_list = ModulePath::new_child(db, core, db.it_ident_borrowed("list").unwrap());
        let core_marker = ModulePath::new_child(db, core, db.it_ident_borrowed("marker").unwrap());
        let core_mem = ModulePath::new_child(db, core, db.it_ident_borrowed("mem").unwrap());
        let core_num = ModulePath::new_child(db, core, db.it_ident_borrowed("num").unwrap());
        let core_ops = ModulePath::new_child(db, core, db.it_ident_borrowed("ops").unwrap());
        let core_option = ModulePath::new_child(db, core, db.it_ident_borrowed("option").unwrap());
        let core_prelude =
            ModulePath::new_child(db, core, db.it_ident_borrowed("prelude").unwrap());
        let core_raw_bits =
            ModulePath::new_child(db, core, db.it_ident_borrowed("raw_bits").unwrap());
        let core_result = ModulePath::new_child(db, core, db.it_ident_borrowed("result").unwrap());
        let core_slice = ModulePath::new_child(db, core, db.it_ident_borrowed("slice").unwrap());
        let core_str = ModulePath::new_child(db, core, db.it_ident_borrowed("str").unwrap());
        let core_visual = ModulePath::new_child(db, core, db.it_ident_borrowed("visual").unwrap());
        Self {
            core_package,
            std_package,
            core_library,
            std_library,
            core_root: core,
            std,
            core_array,
            core_basic,
            core_clone,
            core_default,
            core_marker,
            core_mem,
            core_num,
            core_ops,
            core_option,
            core_prelude,
            core_raw_bits,
            core_result,
            core_slice,
            core_str,
            core_list,
            core_visual,
        }
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

    pub fn core_root(&self) -> ModulePath {
        self.core_root
    }

    pub fn core_basic(&self) -> ModulePath {
        self.core_basic
    }

    pub fn core_clone(&self) -> ModulePath {
        self.core_clone
    }

    pub fn core_default(&self) -> ModulePath {
        self.core_default
    }

    /// core::marker
    pub fn core_marker(&self) -> ModulePath {
        self.core_marker
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

    pub fn core_result(&self) -> ModulePath {
        self.core_result
    }

    /// core::list
    pub fn core_list(&self) -> ModulePath {
        self.core_list
    }

    /// core::array
    pub fn core_array(&self) -> ModulePath {
        self.core_array
    }

    /// core::visual
    pub fn core_visual(&self) -> ModulePath {
        self.core_visual
    }

    pub fn std(&self) -> ModulePath {
        self.std
    }

    pub fn core_ops(&self) -> ModulePath {
        self.core_ops
    }

    pub fn core_option_ty_path(&self) -> ModulePath {
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
    let menu = db.vfs_path_menu(toolchain);
    assert_eq!(menu.core_root().to_string_with_db(&db), "core");
    assert_eq!(menu.std().to_string_with_db(&db), "std");
    assert_eq!(menu.core_basic().to_string_with_db(&db), "core::basic");
    assert_eq!(menu.core_default().to_string_with_db(&db), "core::default");
    assert_eq!(menu.core_mem().to_string_with_db(&db), "core::mem");
    assert_eq!(menu.core_num().to_string_with_db(&db), "core::num");
    assert_eq!(menu.core_slice().to_string_with_db(&db), "core::slice");
    assert_eq!(menu.core_str().to_string_with_db(&db), "core::str");
    assert_eq!(menu.core_ops().to_string_with_db(&db), "core::ops");
    assert_eq!(
        menu.core_option_ty_path().to_string_with_db(&db),
        "core::option"
    );
    assert_eq!(menu.core_prelude().to_string_with_db(&db), "core::prelude");
    assert_eq!(
        menu.core_raw_bits().to_string_with_db(&db),
        "core::raw_bits"
    );
    assert_eq!(menu.core_list().to_string_with_db(&db), "core::list");
    assert_eq!(menu.core_visual().to_string_with_db(&db), "core::visual");
}
