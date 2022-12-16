use husky_entity_path::EntityPathDb;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AbsoluteEntityPath(EntityPath);

impl AbsoluteEntityPath {
    pub fn path(self) -> EntityPath {
        self.0
    }
}

#[salsa::tracked(jar = EntityTreeJar)]
pub(crate) fn absolute_entity_path(
    db: &dyn EntityTreeDb,
    entity_path: EntityPath,
) -> AbsoluteEntityPath {
    todo!()
}

#[test]
fn absolute_entity_path_works() {
    let db = DB::default();
    let menu = db.entity_path_menu();
    assert!(db.is_absolute(menu.b32()));
    assert!(db.is_absolute(menu.b64()));
    assert!(db.is_absolute(menu.i32()));
    assert!(db.is_absolute(menu.i64()));
    assert!(db.is_absolute(menu.f32()));
    assert!(db.is_absolute(menu.f64()));
}
