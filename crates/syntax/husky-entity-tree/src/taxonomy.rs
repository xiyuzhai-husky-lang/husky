use crate::*;
use husky_accessibility::Accessibility;
use husky_entity_path::EntityPathData;

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn entity_class(
    db: &dyn EntityTreeDb,
    entity_path: EntityPath,
) -> EntityTreeResult<EntityCard> {
    let node = entity_node(db, entity_path).as_ref()?;
    Ok(match entity_path.data(db) {
        EntityPathData::CrateRoot(crate_path) => EntityCard::Module,
        EntityPathData::Childpath { parent, ident } => {
            // match pare
            // let crate_path = db.book_crate_of_entity_path(parent);
            // let entity_tree_sheet = db.entity_tree_sheet(crate_path);
            let module = closest_module_in_apparent_ancestry(db, entity_path);
            todo!()
        }
    })
}
