use std::{collections::HashMap, str::FromStr};

use salsa::debug::ExpectWithDb;

use super::*;

pub struct AdHocPackage {
    package_path: PackagePath,
    crate_path: CratePath,
    root_module_path: ModulePath,
    non_root_module_paths: HashMap<&'static str, ModulePath>,
}

impl AdHocPackage {
    pub fn new_lib(
        db: &mut dyn VfsDb,
        lib_content: &'static str,
        non_root_modules: impl IntoIterator<Item = (&'static str, &'static str)>,
    ) -> Self {
        let package_fs_path = PathBuf::from_str("ad_hoc_314").unwrap();
        let package_path = PackagePath::new_local_or_toolchain_package(
            db,
            db.dev_toolchain().unwrap(),
            Name::from_ref(db, "ad-hoc-314").unwrap(),
            &package_fs_path,
        )
        .unwrap();
        db.set_content(
            &package_fs_path.join("src/lib.hsy"),
            FileContent::LiveDoc(lib_content.to_string()),
        )
        .unwrap();
        let crate_path = package_path.lib_crate_path(db).expect("should be okay");
        let non_root_module_paths = HashMap::default();
        for _ in non_root_modules {
            todo!()
        }
        Self {
            package_path,
            crate_path,
            root_module_path: crate_path.root_module_path(db),
            non_root_module_paths,
        }
    }
}

#[test]
fn create_ad_hoc_package_works() {
    let mut db = DB::default();
    AdHocPackage::new_lib(
        &mut db,
        r#"
def a: i32 -> i32:
    x
"#,
        [],
    );
}
