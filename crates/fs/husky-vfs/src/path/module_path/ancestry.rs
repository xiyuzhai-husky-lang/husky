use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::derive_debug_with_db]
pub struct ModuleAncestry {
    crate_path: CratePath,
    module_paths: Vec<ModulePath>,
}

impl ModuleAncestry {
    pub fn crate_path(&self) -> CratePath {
        self.crate_path
    }

    pub fn root_module_path(&self) -> ModulePath {
        self.module_paths[0]
    }

    pub(crate) fn contains(&self, module_path: ModulePath) -> bool {
        self.module_paths.contains(&module_path)
    }
}

#[salsa::tracked(jar = VfsJar, return_ref)]
pub(crate) fn module_ancestry(db: &::salsa::Db, module_path: ModulePath) -> ModuleAncestry {
    match module_path.data(db) {
        ModulePathData::Root(crate_path) => ModuleAncestry {
            crate_path,
            module_paths: vec![module_path],
        },
        ModulePathData::Script { .. } => todo!(),
        ModulePathData::Child { parent, ident: _ } => {
            let mut ancestry = module_ancestry(db, parent).clone();
            ancestry.module_paths.push(module_path);
            ancestry
        }
    }
}

#[test]
fn module_ancestry_works() {
    use crate::tests::*;
    use salsa::DebugWithDb;
    fn t<'a>(db: &'a ::salsa::Db, item_path: ModulePath) -> salsa::DebugWith<'a> {
        module_ancestry(db, item_path).debug(db)
    }

    let db = DB::default();
    let menu = db.dev_path_menu().unwrap();
    expect_test::expect![[r#"
        ModuleAncestry {
            crate_path: CratePath {
                package_path: PackagePath {
                    toolchain: Toolchain {
                        data: ToolchainData::Local {
                            library_path: VirtualPath {
                                _data: VirtualPathBuf(
                                    "../../../library",
                                ),
                            },
                        },
                    },
                    name: `core`,
                    data: PackagePathSource::Library,
                },
                crate_kind: Lib,
            },
            module_paths: [
                `core`,
            ],
        }
    "#]]
    .assert_debug_eq(&t(&db, menu.core_root()));
    expect_test::expect![[r#"
        ModuleAncestry {
            crate_path: CratePath {
                package_path: PackagePath {
                    toolchain: Toolchain {
                        data: ToolchainData::Local {
                            library_path: VirtualPath {
                                _data: VirtualPathBuf(
                                    "../../../library",
                                ),
                            },
                        },
                    },
                    name: `core`,
                    data: PackagePathSource::Library,
                },
                crate_kind: Lib,
            },
            module_paths: [
                `core`,
                `core::basic`,
            ],
        }
    "#]]
    .assert_debug_eq(&t(&db, menu.core_basic().inner()));
}
