use salsa::DebugWithDb;

use crate::*;

#[salsa::interned(jar = EntityPathJar, override_debug)]
pub struct ConnectedModuleItemPath {
    pub module_path: ModulePath,
    pub ident: Identifier,
}

impl ConnectedModuleItemPath {
    pub fn show_aux(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EntityPathDb,
    ) -> std::fmt::Result {
        self.module_path(db).show_aux(f, db)?;
        f.write_str("::")?;
        f.write_str(self.ident(db).data(db))
    }
}

impl<Db: EntityPathDb + ?Sized> salsa::DebugWithDb<Db> for ConnectedModuleItemPath {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as DbWithJar<EntityPathJar>>::as_jar_db(db);
        if include_all_fields {
            todo!()
        } else {
            f.write_str("`")?;
            self.show_aux(f, db)?;
            f.write_str("`")
        }
    }
}

#[test]
fn connected_module_item_path_debug_with_db_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let entity_path_menu = db.entity_path_menu(toolchain).unwrap();
    expect_test::expect![[r#"
        `core::num::r32`
    "#]]
    .assert_debug_eq(&entity_path_menu.r32().debug(&db));
}
