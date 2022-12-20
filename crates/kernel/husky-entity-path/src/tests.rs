use crate::*;
use husky_package_path::PackagePathJar;
use husky_toolchain::*;
use husky_word::WordJar;
use salsa::DebugWithDb;

#[salsa::db(WordJar, ToolchainJar, PackagePathJar, EntityPathJar)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

#[test]
fn entity_path_debug_works() {
    let db = DB::default();
    let toolchain = db.lang_dev_toolchain();
    let entity_path_menu = db.entity_path_menu(toolchain).as_ref().unwrap();
    expect_test::expect![[r#"
        EntityPath {
            [show]: "crate::num::i32",
            [crate]: CratePath {
                [salsa id]: 0,
                package_path: Local {
                    path: AbsolutePath(
                        "/home/xiyuzhai/repos/husky/library/core",
                    ),
                },
                crate_kind: Library,
            },
        }
    "#]]
    .assert_debug_eq(&entity_path_menu.i32().debug(&db));
}
