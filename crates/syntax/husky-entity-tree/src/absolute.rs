use husky_entity_kind::EntityKind;
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
pub(crate) fn absolute_entity_path(
    db: &dyn EntityTreeDb,
    entity_path: EntityPath,
) -> EntityTreeResult<AbsoluteEntityPath> {
    match db.entity_kind(entity_path).as_ref()? {
        EntityKind::Module => todo!(),
        EntityKind::Type(_) => todo!(),
        EntityKind::Trait => todo!(),
        EntityKind::Member(_) => todo!(),
        EntityKind::Function { requires_lazy } => todo!(),
        EntityKind::Feature => todo!(),
        EntityKind::EnumVariant => todo!(),
        EntityKind::Main => todo!(),
    }
    Ok(match entity_path.data(db) {
        EntityPathData::CrateRoot(_) => AbsoluteEntityPath(entity_path),
        EntityPathData::Childpath { parent, ident } => {
            todo!()
        }
    })
}

#[test]
fn absolute_entity_path_works() {
    let db = DB::default();
    let menu = db.entity_path_menu();
    assert!(db.is_absolute(menu.b32()).unwrap());
    assert!(db.is_absolute(menu.b64()).unwrap());
    assert!(db.is_absolute(menu.i32()).unwrap());
    assert!(db.is_absolute(menu.i64()).unwrap());
    assert!(db.is_absolute(menu.f32()).unwrap());
    assert!(db.is_absolute(menu.f64()).unwrap());
}

pub(crate) fn absolutize_parent(
    db: &dyn EntityTreeDb,
    entity_path: EntityPath,
) -> EntityTreeResult<EntityPath> {
    Ok(match entity_path.data(db) {
        EntityPathData::CrateRoot(_) => entity_path,
        EntityPathData::Childpath { parent, ident } => {
            let abs_parent = db.entity_absolute_path(parent).as_ref()?.path();
            if abs_parent == parent {
                entity_path
            } else {
                db.it_entity_path(EntityPathData::Childpath {
                    parent: abs_parent,
                    ident,
                })
            }
        }
    })
}
