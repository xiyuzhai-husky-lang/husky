use husky_entity_card::EntityCard;
use husky_entity_path::{EntityPathData, EntityPathDb};

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AbsoluteEntityPath(EntityPath);

impl AbsoluteEntityPath {
    pub fn path(self) -> EntityPath {
        self.0
    }
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn entity_absolute_path(
    db: &dyn EntityTreeDb,
    entity_path: EntityPath,
) -> EntityTreeResult<AbsoluteEntityPath> {
    Ok(match entity_path.data(db) {
        EntityPathData::Module(_) => AbsoluteEntityPath(entity_path),
        EntityPathData::Associated { parent, ident } => {
            let abs_parent = entity_absolute_path(db, parent).as_ref()?.path();
            if abs_parent != parent {
                todo!()
                // *entity_absolute_path(db, EntityPath::new_child(db, *abs_parent, ident)).as_ref()?
            } else {
                match db.entity_card(entity_path).as_ref()? {
                    EntityCard::Use => todo!(),
                    _ => AbsoluteEntityPath(entity_path),
                }
            }
        }
    })
}

#[test]
fn absolute_entity_path_works() {
    let db = DB::default();
    let toolchain = db.lang_dev_toolchain();
    let menu = db.entity_path_menu(toolchain).unwrap();
    assert!(db.is_absolute(menu.i32()).unwrap());
    assert!(db.is_absolute(menu.i64()).unwrap());
    // todo
    // assert!(db.is_absolute(menu.f32()).unwrap());
    // assert!(db.is_absolute(menu.f64()).unwrap());
    // assert!(db.is_absolute(menu.b32()).unwrap());
    // assert!(db.is_absolute(menu.b64()).unwrap());
}

pub(crate) fn absolutize_parent(
    db: &dyn EntityTreeDb,
    entity_path: EntityPath,
) -> EntityTreeResult<EntityPath> {
    Ok(match entity_path.data(db) {
        EntityPathData::Module(_) => entity_path,
        EntityPathData::Associated { parent, ident } => {
            let abs_parent = db.entity_absolute_path(parent).as_ref()?.path();
            if abs_parent == parent {
                entity_path
            } else {
                todo!()
                // db.it_entity_path(EntityPathData::Associated {
                //     parent: abs_parent,
                //     ident,
                // })
            }
        }
    })
}
