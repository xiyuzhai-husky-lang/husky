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
    let entity_path_menu = db.entity_path_menu(toolchain);
    expect_test::expect![[r#"
        EntityPath {
            [salsa id]: 12,
            data: CrateRoot {
                parent: EntityPath {
                    [salsa id]: 4,
                    data: CrateRoot {
                        parent: EntityPath {
                            [salsa id]: 0,
                            data: CrateRoot(
                                CratePath {
                                    [salsa id]: 0,
                                    package_path: PackagePathData::Builtin {
                                        ident: "core",
                                        toolchain: Toolchain {
                                            [salsa id]: 0,
                                            data: Local {
                                                library_path: "/home/xiyuzhai/repos/husky/library",
                                            },
                                        },
                                    },
                                    crate_kind: Library,
                                },
                            ),
                        },
                        ident: Identifier(
                            Word {
                                [salsa id]: 10,
                                data: "num",
                            },
                        ),
                    },
                },
                ident: Identifier(
                    Word {
                        [salsa id]: 2,
                        data: "i32",
                    },
                ),
            },
        }
    "#]]
    .assert_debug_eq(&entity_path_menu.i32().debug(&db));
}
