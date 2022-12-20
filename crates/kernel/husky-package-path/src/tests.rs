use husky_toolchain::{ToolchainDb, ToolchainJar};
use husky_word::WordJar;

use crate::*;

#[salsa::db(WordJar, ToolchainJar, PackagePathJar)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

#[test]
fn package_path_debug_works() {
    let db = DB::default();
    let toolchain = db.lang_dev_toolchain();
    let package_path_menu = db.package_path_menu(toolchain);
    expect_test::expect![[r#"
        PackagePathData::Builtin {
            ident: "core",
            toolchain: Toolchain {
                [salsa id]: 0,
                data: Local {
                    library_path: "/home/xiyuzhai/repos/husky/library",
                },
            },
        }
    "#]].assert_debug_eq(&package_path_menu.core().debug(&db));
}
