use crate::*;
use husky_accessibility::Accessibility;
use husky_entity_path::EntityPathData;

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn visibility(
    db: &dyn EntityTreeDb,
    entity_path: EntityPath,
) -> EntityTreeResult<Accessibility> {
    Ok(match entity_path.data(db) {
        EntityPathData::CrateRoot(crate_path) => Accessibility::Public,
        EntityPathData::Childpath { parent, ident } => {
            todo!()
        }
    })
}
