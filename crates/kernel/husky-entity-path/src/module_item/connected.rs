use salsa::DebugWithDb;

use crate::*;

#[salsa::interned(jar = EntityPathJar)]
pub struct ConnectedModuleItemPath {
    pub module_path: ModulePath,
    pub ident: Identifier,
}

impl<Db: EntityPathDb> salsa::DebugWithDb<Db> for ConnectedModuleItemPath {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        self.fmt(f, db as &dyn EntityPathDb, include_all_fields)
    }
}

#[test]
fn connected_module_item_path_debug_with_db_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let entity_path_menu = db.entity_path_menu(toolchain).unwrap();
    expect_test::expect![[r#"
        ConnectedModuleItemPath {
            module_path: `core::num`,
            ident: Identifier(
                Word(
                    Id {
                        value: 7,
                    },
                ),
            ),
        }
    "#]]
    .assert_debug_eq(&entity_path_menu.b32().debug(&db));
}
