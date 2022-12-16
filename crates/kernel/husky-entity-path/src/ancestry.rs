use crate::*;

#[salsa::tracked(jar = EntityPathJar, return_ref)]
pub(crate) fn ancestry(db: &dyn EntityPathDb, entity_path: EntityPath) -> Vec<EntityPath> {
    match db.dt_entity_path(entity_path) {
        EntityPathData::Crate { package, kind } => vec![entity_path],
        EntityPathData::Childpath { parent, ident } => {
            let mut ancestry = ancestry(db, parent).clone();
            ancestry.push(entity_path);
            ancestry
        }
    }
}

#[test]
fn ancestry_works() {
    use crate::tests::*;
    fn t(db: &DB, entity_path: EntityPath) -> Vec<String> {
        ancestry(db, entity_path)
            .iter()
            .map(|path| path.display(db))
            .collect()
    }

    let db = DB::default();
    let menu = db.entity_path_menu();
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
