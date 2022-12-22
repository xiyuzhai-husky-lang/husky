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
        include_all_fields: bool,
    ) -> std::fmt::Result {
        f.debug_struct("ModuleAncestry")
            .field(
                "crate_path",
                &self.crate_path.debug_with(db, include_all_fields),
            )
            .field(
                "modules",
                &self.module_paths.debug_with(db, include_all_fields),
            )
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
    let toolchain = db.dev_toolchain();
    let menu = db.path_menu(toolchain).unwrap();
    expect_test::expect![[r#"
        ModuleAncestry {
            crate_path: CratePath {
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
            modules: [
                ModulePath {
                    [display]: "crate",
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
            ],
        }
    "#]]
    .assert_debug_eq(&t(&db, menu.core()));
    expect_test::expect![[r#"
        ModuleAncestry {
            crate_path: CratePath {
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
            modules: [
                ModulePath {
                    [display]: "crate",
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
                ModulePath {
                    [display]: "crate::basic",
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
            ],
        }
    "#]]
    .assert_debug_eq(&t(&db, menu.core_basic()));
}
