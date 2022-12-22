use crate::*;
use husky_word::WordJar;
use salsa::DebugWithDb;

#[salsa::db(WordJar, VfsJar, EntityPathJar)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

#[test]
fn entity_path_debug_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain();
    let entity_path_menu = db.entity_path_menu(toolchain).unwrap();
    expect_test::expect![[r#"
        ModuleItemPath {
            [salsa id]: 14,
            module: ModulePath {
                [display]: "crate::num",
                [crate]: CratePath {
                    [salsa id]: 0,
                    package_path: PackagePath {
                        [salsa id]: 0,
                        toolchain: Toolchain {
                            [salsa id]: 0,
                            data: Local {
                                library_path: DiffPath {
                                    [salsa id]: 0,
                                    data: DiffPathBuf(
                                        "../../../library",
                                    ),
                                },
                            },
                        },
                        data: Local {
                            path: DiffPath {
                                [salsa id]: 1,
                                data: DiffPathBuf(
                                    "../../../library/core",
                                ),
                            },
                        },
                    },
                    crate_kind: Library,
                },
            },
            ident: Identifier(
                "i32",
            ),
        }
    "#]]
    .assert_debug_eq(&entity_path_menu.i32().debug(&db));
}
