use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct VfsPathMenu {
    core_package: PackagePath,
    std_package: PackagePath,
    core_library: CratePath,
    std_library: CratePath,
    /// core
    core_root: ModulePath,
    std_root: ModulePath,
    core_array: SubmodulePath,
    core_basic: SubmodulePath,
    core_clone: SubmodulePath,
    core_default: SubmodulePath,
    core_list: SubmodulePath,
    core_marker: SubmodulePath,
    core_mem: SubmodulePath,
    /// core::num
    core_num: SubmodulePath,
    core_slice: SubmodulePath,
    core_str: SubmodulePath,
    core_ops: SubmodulePath,
    core_option: SubmodulePath,
    core_prelude: SubmodulePath,
    core_raw_bits: SubmodulePath,
    core_result: SubmodulePath,
    core_visual: SubmodulePath,
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
        let core_root = ModulePath::new_root(db, core_library);
        let std_root = ModulePath::new_root(db, std_library);
        // todo: refactor with it_ident_borrowed_uncheck or Ident::from_borrowed_unchecked
        let core_array = ModulePath::new_child(
            db,
            core_root,
            db.it_ident_borrowed("array")
                .expect("should be valid identifier"),
        );
        let core_basic = ModulePath::new_child(
            db,
            core_root,
            db.it_ident_borrowed("basic")
                .expect("should be valid identifier"),
        );
        let core_clone = ModulePath::new_child(
            db,
            core_root,
            db.it_ident_borrowed("clone")
                .expect("should be valid identifier"),
        );
        let core_default = ModulePath::new_child(
            db,
            core_root,
            db.it_ident_borrowed("default")
                .expect("should be valid identifier"),
        );
        let core_list = ModulePath::new_child(
            db,
            core_root,
            db.it_ident_borrowed("list")
                .expect("should be valid identifier"),
        );
        let core_marker = ModulePath::new_child(
            db,
            core_root,
            db.it_ident_borrowed("marker")
                .expect("should be valid identifier"),
        );
        let core_mem = ModulePath::new_child(
            db,
            core_root,
            db.it_ident_borrowed("mem")
                .expect("should be valid identifier"),
        );
        let core_num = ModulePath::new_child(
            db,
            core_root,
            db.it_ident_borrowed("num")
                .expect("should be valid identifier"),
        );
        let core_ops = ModulePath::new_child(
            db,
            core_root,
            db.it_ident_borrowed("ops")
                .expect("should be valid identifier"),
        );
        let core_option = ModulePath::new_child(
            db,
            core_root,
            db.it_ident_borrowed("option")
                .expect("should be valid identifier"),
        );
        let core_prelude = ModulePath::new_child(
            db,
            core_root,
            db.it_ident_borrowed("prelude")
                .expect("should be valid identifier"),
        );
        let core_raw_bits = ModulePath::new_child(
            db,
            core_root,
            db.it_ident_borrowed("raw_bits")
                .expect("should be valid identifier"),
        );
        let core_result = ModulePath::new_child(
            db,
            core_root,
            db.it_ident_borrowed("result")
                .expect("should be valid identifier"),
        );
        let core_slice = ModulePath::new_child(
            db,
            core_root,
            db.it_ident_borrowed("slice")
                .expect("should be valid identifier"),
        );
        let core_str = ModulePath::new_child(
            db,
            core_root,
            db.it_ident_borrowed("str")
                .expect("should be valid identifier"),
        );
        let core_visual = ModulePath::new_child(
            db,
            core_root,
            db.it_ident_borrowed("visual")
                .expect("should be valid identifier"),
        );
        Self {
            core_package,
            std_package,
            core_library,
            std_library,
            core_root,
            std_root,
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

    pub fn core_basic(&self) -> SubmodulePath {
        self.core_basic
    }

    pub fn core_clone(&self) -> SubmodulePath {
        self.core_clone
    }

    pub fn core_default(&self) -> SubmodulePath {
        self.core_default
    }

    /// core::marker
    pub fn core_marker(&self) -> SubmodulePath {
        self.core_marker
    }

    /// core::mem
    pub fn core_mem(&self) -> SubmodulePath {
        self.core_mem
    }

    /// core::num
    pub fn core_num(&self) -> SubmodulePath {
        self.core_num
    }

    pub fn core_prelude(&self) -> SubmodulePath {
        self.core_prelude
    }

    /// core::raw_bits
    pub fn core_raw_bits(&self) -> SubmodulePath {
        self.core_raw_bits
    }

    pub fn core_result(&self) -> SubmodulePath {
        self.core_result
    }

    /// core::list
    pub fn core_list(&self) -> SubmodulePath {
        self.core_list
    }

    /// core::array
    pub fn core_array(&self) -> SubmodulePath {
        self.core_array
    }

    /// core::visual
    pub fn core_visual(&self) -> SubmodulePath {
        self.core_visual
    }

    pub fn std_root(&self) -> ModulePath {
        self.std_root
    }

    pub fn core_ops(&self) -> SubmodulePath {
        self.core_ops
    }

    pub fn core_option(&self) -> SubmodulePath {
        self.core_option
    }

    pub fn core_slice(&self) -> SubmodulePath {
        self.core_slice
    }

    pub fn core_str(&self) -> SubmodulePath {
        self.core_str
    }
}

#[test]
fn vfs_path_menu_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let menu = db.vfs_path_menu(toolchain);
    assert_eq!(menu.core_root().to_string_with_db(&db), "core");
    assert_eq!(menu.std_root().to_string_with_db(&db), "std");
    assert_eq!(menu.core_basic().to_string_with_db(&db), "core::basic");
    assert_eq!(menu.core_default().to_string_with_db(&db), "core::default");
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
    assert_eq!(menu.core_list().to_string_with_db(&db), "core::list");
    assert_eq!(menu.core_visual().to_string_with_db(&db), "core::visual");
}
