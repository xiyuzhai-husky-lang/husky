use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleAncestry {
    crate_path: CratePath,
    module_paths: Vec<ModulePath>,
}

impl ModuleAncestry {
    pub fn crate_path(&self) -> CratePath {
        self.crate_path
    }

    pub(crate) fn contains(&self, module_path: ModulePath) -> bool {
        self.module_paths.contains(&module_path)
    }
}

impl<Db: VfsDb> salsa::DebugWithDb<Db> for ModuleAncestry {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DebugFormatLevel,
    ) -> std::fmt::Result {
        f.debug_struct("ModuleAncestry")
            .field("crate_path", &self.crate_path.debug_with(db, level.next()))
            .field("modules", &self.module_paths.debug_with(db, level.next()))
            .finish()
    }
}

#[salsa::tracked(jar = VfsJar, return_ref)]
pub(crate) fn module_ancestry(db: &dyn VfsDb, module_path: ModulePath) -> ModuleAncestry {
    match module_path.data(db) {
        ModulePathData::Root(crate_path) => ModuleAncestry {
            crate_path,
            module_paths: vec![module_path],
        },
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
    fn t<'a>(db: &'a DB, entity_path: ModulePath) -> salsa::DebugWith<'a, DB> {
        module_ancestry(db, entity_path).debug(db)
    }

    let db = DB::default();
    let menu = db.dev_path_menu().unwrap();
    expect_test::expect![[r#"
        ModuleAncestry {
            crate_path: CratePath {
                package_path: PackagePath {
                    toolchain: Toolchain {
                        data: ToolchainData::Local {
                            library_path: DiffPath {
                                data: DiffPathBuf(
                                    "../../../library",
                                ),
                            },
                        },
                    },
                    name: Name(
                        Word(
                            Id {
                                value: 1,
                            },
                        ),
                    ),
                    data: PackagePathSource::Toolchain,
                },
                crate_kind: Library,
            },
            modules: [
                `core`,
            ],
        }
    "#]]
    .assert_debug_eq(&t(&db, menu.core()));
    expect_test::expect![[r#"
        ModuleAncestry {
            crate_path: CratePath {
                package_path: PackagePath {
                    toolchain: Toolchain {
                        data: ToolchainData::Local {
                            library_path: DiffPath {
                                data: DiffPathBuf(
                                    "../../../library",
                                ),
                            },
                        },
                    },
                    name: Name(
                        Word(
                            Id {
                                value: 1,
                            },
                        ),
                    ),
                    data: PackagePathSource::Toolchain,
                },
                crate_kind: Library,
            },
            modules: [
                `core`,
                `core::basic`,
            ],
        }
    "#]]
    .assert_debug_eq(&t(&db, menu.core_basic()));
}
