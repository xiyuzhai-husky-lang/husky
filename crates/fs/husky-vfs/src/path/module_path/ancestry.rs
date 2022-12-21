use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApparentAncestry {
    crate_path: CratePath,
    modules: Vec<ModulePath>,
}

impl ApparentAncestry {
    pub fn crate_path(&self) -> CratePath {
        self.crate_path
    }
}

#[salsa::tracked(jar = VfsJar, return_ref)]
pub(crate) fn apparent_ancestry(db: &dyn VfsDb, module_path: ModulePath) -> ApparentAncestry {
    match module_path.data(db) {
        ModulePathData::Root(crate_path) => ApparentAncestry {
            crate_path,
            modules: vec![module_path],
        },
        ModulePathData::Child { parent, ident: _ } => {
            let mut ancestry = apparent_ancestry(db, parent).clone();
            ancestry.modules.push(module_path);
            ancestry
        }
    }
}

#[test]
fn apparent_ancestry_works() {
    use crate::tests::*;
    fn t(db: &DB, entity_path: ModulePath) -> Vec<String> {
        todo!()
        // apparent_ancestry(db, entity_path)
        //     .modules
        //     .iter()
        //     .map(|path| path.show(db))
        //     .collect()
    }

    let db = DB::default();
    let toolchain = db.lang_dev_toolchain();
    let menu = db.path_menu(toolchain).unwrap();
    expect_test::expect![[r#"
        [
            "crate",
        ]
    "#]]
    .assert_debug_eq(&t(&db, menu.core()));
    expect_test::expect![[r#"
        [
            "crate",
        ]
    "#]]
    .assert_debug_eq(&t(&db, menu.core_basic()));
}
