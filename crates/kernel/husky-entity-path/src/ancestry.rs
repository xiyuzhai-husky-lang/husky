use husky_toolchain::ToolchainDb;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApparentAncestry {
    crate_path: CratePath,
    paths: Vec<EntityPath>,
}

impl ApparentAncestry {
    pub fn crate_path(&self) -> CratePath {
        self.crate_path
    }
}

#[salsa::tracked(jar = EntityPathJar, return_ref)]
pub(crate) fn apparent_ancestry(
    db: &dyn EntityPathDb,
    entity_path: EntityPath,
) -> ApparentAncestry {
    match entity_path.data(db) {
        EntityPathData::CrateRoot(crate_path) => ApparentAncestry {
            crate_path,
            paths: vec![entity_path],
        },
        EntityPathData::Childpath { parent, ident: _ } => {
            let mut ancestry = apparent_ancestry(db, parent).clone();
            ancestry.paths.push(entity_path);
            ancestry
        }
    }
}

#[test]
fn apparent_ancestry_works() {
    use crate::tests::*;
    fn t(db: &DB, entity_path: EntityPath) -> Vec<String> {
        apparent_ancestry(db, entity_path)
            .paths
            .iter()
            .map(|path| path.show(db))
            .collect()
    }

    let db = DB::default();
    let toolchain = db.lang_dev_toolchain();
    let menu = db.entity_path_menu(toolchain).as_ref().unwrap();
    expect_test::expect![[r#"
        [
            "crate",
        ]
    "#]]
    .assert_debug_eq(&t(&db, menu.core()));
    expect_test::expect![[r#"
        [
            "crate",
            "crate::num",
            "crate::num::i32",
        ]
    "#]]
    .assert_debug_eq(&t(&db, menu.i32()));
    expect_test::expect![[r#"
        [
            "crate",
            "crate::num",
            "crate::num::i64",
        ]
    "#]]
    .assert_debug_eq(&t(&db, menu.i64()));
    expect_test::expect![[r#"
        [
            "crate",
            "crate::num",
            "crate::num::b32",
        ]
    "#]]
    .assert_debug_eq(&t(&db, menu.b32()));
    expect_test::expect![[r#"
        [
            "crate",
            "crate::num",
            "crate::num::b64",
        ]
    "#]]
    .assert_debug_eq(&t(&db, menu.b64()));
}
